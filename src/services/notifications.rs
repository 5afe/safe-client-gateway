use crate::models::service::notifications::NotificationRegistrationRequest;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

pub async fn post_registration(
    context: Context<'_>,
    registration_request: NotificationRegistrationRequest,
) -> ApiResult<content::Json<String>> {
    Ok(content::Json(String::new()))
}
