use crate::config::default_request_timeout;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::notifications::backend::NotificationRegistrationRequest as BackendRegistrationRequest;
use crate::routes::notifications::models::{
    DeviceData, NotificationRegistrationRequest, SafeRegistration,
};
use crate::utils::context::Context;
use crate::utils::errors::{ApiError, ApiResult};
use serde_json::json;
use serde_json::value::RawValue;
use serde_json::{self, value::Value};
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

    let (error_chain_ids, error_body) = {
        let mut error_chain_ids: Vec<&str> = vec![];
        let mut errors: Vec<Value> = vec![];
        for (chain_id, request) in requests.into_iter() {
            match request.await {
                Ok(response) => {
                    if !response.status().is_success() {
                        error_chain_ids.push(chain_id);
                        errors.push(json!({
                            chain_id : RawValue::from_string(response.text().await.expect("Error response issue"))?
                            }
                        ))
                    }
                }
                Err(reqwest_error) => {
                    error_chain_ids.push(chain_id);
                    errors.push(json!({
                        chain_id : serde_json::to_value(reqwest_error.to_string())?
                    }))
                }
            }
        }
        (error_chain_ids, json!(errors))
    };

    if error_chain_ids.is_empty() {
        Ok(())
    } else {
        Err(ApiError::new_from_message_with_debug(
            format!(
                "Push notification registration failed for chain IDs: {}",
                error_chain_ids.join(", ")
            ),
            Some(error_body),
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
