use crate::models::backend::notifications::{
    DeviceData, NotificationRegistrationRequest as BackendRegistrationRequest,
    NotificationRegistrationResult,
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
    let mut results: Vec<NotificationRegistrationResult> = vec![];

    for safe_registration in registration_request.safe_registrations.iter() {
        let info_provider = DefaultInfoProvider::new(&safe_registration.chain_id, &context);
        let url = core_uri!(info_provider, "/notification/devices")?;
        let backend_request =
            build_backend_request(&registration_request.device_data, safe_registration);
        let response = client
            .post(url.to_string())
            .json(&backend_request)
            .send()
            .await?;

        results.push(serde_json::from_str(&response.text().await?)?);
    }

    Ok(content::Json(serde_json::to_string(&results)?))
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
