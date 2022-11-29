use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::http_client::{Request, Response};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[post(
    "/v1/chains/<chain_id>/safes/<safe_address>/messages",
    format = "application/json",
    data = "<message_payload>"
)]
pub async fn route(
    context: RequestContext,
    chain_id: String,
    safe_address: String,
    message_payload: Json<CreateMessage>,
) -> ApiResult<String> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    let url = core_uri!(info_provider, "/v1/safes/{}/messages/", safe_address)?;

    let request = {
        let mut request = Request::new(url);
        request.body(serde_json::to_string(&message_payload.0).ok());
        request
    };
    let response_body: Response = context.http_client().post(request).await?;
    return Ok(response_body.body);
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessage {
    message: String,
    safe_app_id: Option<u64>,
    signature: String,
}
