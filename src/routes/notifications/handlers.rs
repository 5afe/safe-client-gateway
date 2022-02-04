use itertools::Itertools;
use serde_json::json;
use serde_json::value::RawValue;
use serde_json::{self, value::Value};

use crate::common::models::backend::notifications::NotificationRegistrationRequest as BackendRegistrationRequest;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::notifications::models::{
    DeviceData, NotificationRegistrationRequest, SafeRegistration,
};
use crate::utils::context::RequestContext;
use crate::utils::errors::{ApiError, ApiResult, ErrorDetails};
use crate::utils::http_client::Request;

pub async fn delete_registration(
    context: &RequestContext,
    chain_id: String,
    uuid: String,
    safe_address: String,
) -> ApiResult<()> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    let url = core_uri!(
        info_provider,
        "/v1/notifications/devices/{}/safes/{}/",
        uuid,
        safe_address
    )?;

    let request = Request::new(url);
    context.http_client().delete(request).await?;

    Ok(())
}

struct NotificationRegistrationError {
    status_code: u16,
    chain_id: String,
    error: Value,
}

pub async fn post_registration(
    context: &RequestContext,
    registration_request: NotificationRegistrationRequest,
) -> ApiResult<()> {
    let client = context.http_client();
    let mut requests = Vec::with_capacity(registration_request.safe_registrations.len());

    for safe_registration in registration_request.safe_registrations.iter() {
        let info_provider = DefaultInfoProvider::new(&safe_registration.chain_id, &context);
        let url = core_uri!(info_provider, "/v1/notifications/devices/")?;
        let backend_request =
            build_backend_request(&registration_request.device_data, safe_registration);

        let request = {
            let mut request = Request::new(url);
            request.body(Some(serde_json::to_string(&backend_request)?));
            request
        };
        requests.push((&safe_registration.chain_id, client.post(request)));
    }

    let mut errors: Vec<NotificationRegistrationError> = vec![];
    for (chain_id, request) in requests.into_iter() {
        match request.await {
            Err(api_error) => {
                let notification_registration_error = NotificationRegistrationError {
                    status_code: api_error.status,
                    chain_id: String::from(chain_id),
                    error: json!(
                            {chain_id :   RawValue::from_string(api_error.details.message.unwrap_or(String::from("Unknown notification registration issue")))?}),
                };
                errors.push(notification_registration_error);
            }
            _ => {}
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        let has_server_error = errors
            .iter()
            .any(|error| (500..600).contains(&error.status_code));
        let error_chain_ids = errors.iter().map(|error| &error.chain_id).join(", ");
        let json_error = errors.iter().map(|error| &error.error).collect::<Vec<_>>();

        let error = ApiError {
            // This is decoupled from the error collection so it is assuming that any value in [errors]
            // is between 400 and 600. If server errors are found we return 500. Else we return 400
            status: if has_server_error { 500 } else { 400 },
            details: ErrorDetails {
                code: 1337,
                message: Some(format!(
                    "Push notification registration failed for chain IDs: {}",
                    error_chain_ids
                )),
                arguments: None,
                debug: Some(json!(json_error)),
            },
        };
        Err(error)
    }
}

pub fn build_backend_request(
    device_data: &DeviceData,
    safe_registration: &SafeRegistration,
) -> BackendRegistrationRequest {
    BackendRegistrationRequest {
        notification_device_data: device_data.clone(),
        safes: safe_registration.safes.to_owned(),
        signatures: safe_registration.signatures.to_owned(),
    }
}
