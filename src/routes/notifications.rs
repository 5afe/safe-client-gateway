use crate::utils::context::Context;

use crate::models::service::notifications::NotificationRegistrationRequest;
use crate::services::notifications::{delete_registration, post_registration};
use crate::utils::errors::ApiResult;
use rocket::response::content;
use rocket::serde::json::Error;
use rocket::serde::json::Json;

/**
 * `/v1/chains/<chain_id>/transactions/<safe_tx_hash>/confirmations` <br />
 * Returns [TransactionDetails](crate::models::service::transactions::details::TransactionDetails)
 *
 * # Transaction Confirmation
 *
 * This endpoint provides a way for submitting confirmations for clients making use of the `safe_tx_hash` as part of the path, and the very same `safe_tx_hash` signed by an owner corresponding to the safe from which the transaction is being sent.
 *
 * If the confirmation is submitted successfully to the core services, then the local cache for that specific transaction is invalidated and the updated transaction details with the confirmation are returned in the request.
 *
 * ## Path
 *
 * `POST /v1/chains/<chain_id>/transactions/<safe_tx_hash>/confirmations`
 *
 * The expected [crate::models::service::transactions::requests::ConfirmationRequest] body for this request, as well as the returned [crate::models::service::transactions::details::TransactionDetails]
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

#[delete("/v1/chains/<chain_id>/notifications/devices/<uuid>/safes/<safe_address>")]
pub async fn delete_notification_registration(
    context: Context<'_>,
    chain_id: String,
    uuid: String,
    safe_address: String,
) -> ApiResult<()> {
    delete_registration(context, chain_id, uuid, safe_address).await
}
