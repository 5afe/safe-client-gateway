use crate::config::default_request_timeout;
use crate::models::backend::notifications::NotificationRegistrationRequest as BackendRegistrationRequest;
use crate::models::service::notifications::{
    DeviceData, NotificationRegistrationRequest, SafeRegistration,
};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::{ApiError, ApiResult};
use itertools::Itertools;
use std::collections::HashMap;
use std::time::Duration;

pub async fn delete_registration(
    context: Context<'_>,
    chain_id: String,
    uuid: String,
    safe_address: String,
) -> ApiResult<()> {
    let client = context.client();

    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    let url = core_uri!(
        info_provider,
        "/v1/notifications/devices/{}/safes/{}/",
        uuid,
        safe_address
    )?;

    client
        .delete(url)
        .timeout(Duration::from_millis(default_request_timeout()))
        .send()
        .await?;

    Ok(())
}

pub async fn post_registration(
    context: Context<'_>,
    registration_request: NotificationRegistrationRequest,
) -> ApiResult<()> {
    let client = context.client();
    let mut requests = Vec::with_capacity(registration_request.safe_registrations.len());

    for safe_registration in registration_request.safe_registrations.iter() {
        let info_provider = DefaultInfoProvider::new(&safe_registration.chain_id, &context);
        let url = core_uri!(info_provider, "/v1/notifications/devices/")?;
        let backend_request =
            build_backend_request(&registration_request.device_data, safe_registration);

        requests.push((
            &safe_registration.chain_id,
            client
                .post(url.to_string())
                .json(&backend_request)
                .timeout(Duration::from_millis(default_request_timeout()))
                .send(),
        ));
    }

    let chain_id_errors = {
        let mut output: HashMap<&str, String> = Default::default();
        for (chain_id, request) in requests.into_iter() {
            match request.await {
                Ok(response) => {
                    if !response.status().is_success() {
                        output.insert(
                            chain_id,
                            response.text().await.expect("Error response issue"),
                        );
                    }
                }
                Err(reqwest_error) => {
                    output.insert(chain_id, reqwest_error.to_string());
                }
            }
        }
        output
    };

    if chain_id_errors.is_empty() {
        Ok(())
    } else {
        let mapped_errors = chain_id_errors
            .iter()
            .map(|(chain_id, error)| {
                format!(
                    "{} : {}",
                    &chain_id,
                    serde_json::to_string(&error)
                        .expect("Error deserializing error details")
                        .replace("\\", "")
                )
            })
            .collect::<Vec<String>>();
        Err(ApiError::new_from_message_with_arguments(
            format!(
                "Push notification registration failed for chain IDs: {}",
                chain_id_errors.keys().join(", ")
            ),
            Some(mapped_errors.to_vec()),
        ))
    }
}

fn build_backend_request(
    device_data: &DeviceData,
    safe_registration: &SafeRegistration,
) -> BackendRegistrationRequest {
    BackendRegistrationRequest {
        notification_device_data: device_data.clone(),
        safes: safe_registration.safes.to_owned(),
        signatures: safe_registration.signatures.to_owned(),
    }
}
