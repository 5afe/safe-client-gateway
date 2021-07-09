use crate::models::backend::notifications::NotificationRegistrationRequest as BackendRegistrationRequest;
use crate::models::service::notifications::{
    DeviceData, NotificationRegistrationRequest, SafeRegistration,
};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::futures::{self, stream, StreamExt, TryFuture, TryStream, TryStreamExt};

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

    client.delete(url).send().await?;

    Ok(())
}

pub async fn post_registration(
    context: Context<'_>,
    registration_request: NotificationRegistrationRequest,
) -> ApiResult<()> {
    let client = context.client();

    let stream = stream::iter(registration_request.safe_registrations.into_iter());
    stream
        .try_for_each_concurrent(32, |safe_registration| async move {
            let info_provider = DefaultInfoProvider::new(&safe_registration.chain_id, &context);
            let url = core_uri!(info_provider, "/v1/notifications/devices/")?;
            let backend_request =
                build_backend_request(&registration_request.device_data, safe_registration);
            client
                .post(url.to_string())
                .json(&backend_request)
                .send()
                .await?;
            Ok(())
        })
        .await?;

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
