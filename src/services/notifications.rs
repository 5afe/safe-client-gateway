use crate::models::backend::notifications::{
    DeviceData, NotificationRegistrationRequest as BackendRegistrationRequest,
};
use crate::models::service::notifications::{NotificationRegistrationRequest, SafeRegistration};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;
pub async fn post_registration(
    context: Context<'_>,
    registration_request: NotificationRegistrationRequest,
) -> ApiResult<content::Json<String>> {
    let client = context.client();

    for safe_registration in registration_request.safe_registrations.iter() {
        let info_provider = DefaultInfoProvider::new(&safe_registration.chain_id, &context);
        let url = core_uri!(info_provider, "/notification/devices")?;
        let backend_request =
            build_backend_request(&registration_request.device_data, safe_registration);
        // client.post(url.to_string())?.await;

        log::error!("URL: {:#? }", url);
        log::error!("backend request: {:#? }", backend_request);
    }

    Ok(content::Json(String::new()))
}

fn build_backend_request(
    device_data: &DeviceData,
    safe_registration: &SafeRegistration,
) -> BackendRegistrationRequest {
    BackendRegistrationRequest {
        notification_device_data: device_data.clone(),
        safes: safe_registration.safes.to_owned(),
        signatures: vec![safe_registration.signatures.clone()],
    }
}
