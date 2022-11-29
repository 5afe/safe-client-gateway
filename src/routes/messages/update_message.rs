use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::http_client::{Request, Response};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[post(
    "/v1/chains/<chain_id>/messages/<message_hash>/signatures",
    format = "application/json",
    data = "<signature_payload>"
)]
pub async fn route(
    context: RequestContext,
    chain_id: String,
    message_hash: String,
    signature_payload: Json<UpdateMessage>,
) -> ApiResult<String> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    let url = core_uri!(info_provider, "/v1/messages/{}/signatures/", &message_hash)?;

    let request = {
        let mut request = Request::new(url);
        request.body(serde_json::to_string(&signature_payload.0).ok());
        request
    };
    let response_body: Response = context.http_client().post(request).await?;
    return Ok(response_body.body);
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMessage {
    signature: String,
}
