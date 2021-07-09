use crate::config::default_request_timeout;
use crate::models::backend::notifications::NotificationRegistrationRequest as BackendRegistrationRequest;
use crate::models::service::notifications::{
    DeviceData, NotificationRegistrationRequest, SafeRegistration,
};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use std::time::Duration;
// use rocket::futures::{self, stream, StreamExt, TryFuture, TryStream, TryStreamExt};

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

    for safe_registration in registration_request.safe_registrations.into_iter() {
        let info_provider = DefaultInfoProvider::new(&safe_registration.chain_id, &context);
        let url = core_uri!(info_provider, "/v1/notifications/devices/")?;
        let backend_request =
            build_backend_request(&registration_request.device_data, safe_registration);

        requests.push(
            client
                .post(url.to_string())
                .json(&backend_request)
                .timeout(Duration::from_millis(default_request_timeout()))
                .send(),
        );
    }

    for request in requests.into_iter() {
        request.await?;
    }

    Ok(())
}

fn build_backend_request(
    device_data: &DeviceData,
    safe_registration: SafeRegistration,
) -> BackendRegistrationRequest {
    BackendRegistrationRequest {
        notification_device_data: device_data.clone(),
        safes: safe_registration.safes,
        signatures: safe_registration.signatures,
    }
}
