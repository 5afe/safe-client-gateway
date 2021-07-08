use crate::utils::context::Context;

use crate::models::service::notifications::NotificationRegistrationRequest;
use crate::services::notifications::{delete_registration, post_registration};
use crate::utils::errors::ApiResult;
use rocket::serde::json::Error;
use rocket::serde::json::Json;

/**
 * `/v1/register/notifications` <br />
 * Returns `()`
 *
 * # Register notifications
 *
 * This endpoint provides a way for registering devices for push notifications.
 *
 * One can subscribe to as many safes in different chains as [SafeRegistration](crate::models::service::notifications::SafeRegistration) provided in the payload
 *
 * ## Path
 *
 * `POST /v1/register/notifications`
 *
 * ## Query parameters
 *
 * No query parameters available for this endpoint.
 */
#[post(
    "/v1/register/notifications",
    format = "application/json",
    data = "<registration_request>"
)]
pub async fn post_notification_registration<'e>(
    context: Context<'_>,
    registration_request: Result<Json<NotificationRegistrationRequest>, Error<'e>>,
) -> ApiResult<()> {
    post_registration(context, registration_request?.0).await
}

/**
 * `/v1/chains/<chain_id>/notifications/devices/<uuid>/safes/<safe_address>` <br />
 * Returns `()`
 *
 * # Unregister notifications
 *
 * This endpoint provides a way to unsubscribe from push notifications for a given `uuid`.
 *
 * Clients are expected to manage the `uuid` provided originally to the backend.
 *
 * ## Path
 *
 * `DELETE /v1/chains/<chain_id>/notifications/devices/<uuid>/safes/<safe_address>`
 *
 * ## Query parameters
 *
 * No query parameters available for this endpoint.
 */
#[delete("/v1/chains/<chain_id>/notifications/devices/<uuid>/safes/<safe_address>")]
pub async fn delete_notification_registration(
    context: Context<'_>,
    chain_id: String,
    uuid: String,
    safe_address: String,
) -> ApiResult<()> {
    delete_registration(context, chain_id, uuid, safe_address).await
}
