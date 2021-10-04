use crate::config::default_request_timeout;
use crate::models::backend::notifications::NotificationRegistrationRequest as BackendRegistrationRequest;
use crate::models::service::notifications::{
    DeviceData, NotificationRegistrationRequest, SafeRegistration,
};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::{ApiError, ApiResult, ErrorDetails};
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

    let response = client
        .delete(url)
        .timeout(Duration::from_millis(default_request_timeout()))
        .send()
        .await?;

    forward_error(response).await
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
        let mut output: Vec<(&str, ErrorDetails)> = vec![];
        for (chain_id, request) in requests.into_iter() {
            match request.await {
                Ok(response) => {
                    if let Err(response_error) = forward_error(response).await {
                        output.push((chain_id, response_error.details))
                    }
                }
                Err(reqwest_error) => {
                    let api_error: ApiError = reqwest_error.into();
                    output.push((chain_id, api_error.details))
                }
            }
        }
        output
    };

    if chain_id_errors.is_empty() {
        Ok(())
    } else {
        let mapped_errors = chain_id_errors
            .into_iter()
            .map(|(chain_id, error)| {
                format!("{:#?} : {:#?}", &chain_id, serde_json::to_string(&error))
            })
            .collect::<Vec<String>>();
        Err(ApiError::new_from_message_with_arguments(
            "Push notification registration failed for chain ids: {}",
            Some(mapped_errors),
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

async fn forward_error(response: reqwest::Response) -> ApiResult<()> {
    if response.status().is_success() {
        Ok(())
    } else {
        Err(ApiError::from_http_response(
            response,
            String::from("Unexpected notification registration error"),
        )
        .await)
    }
}
