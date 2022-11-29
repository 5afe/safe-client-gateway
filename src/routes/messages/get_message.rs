use super::frontend_models::Message as FrontendMessage;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::messages::backend_models::Message;
use crate::routes::messages::message_mapper::map_message;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::http_client::Request;
use rocket::response::content;

#[get("/v1/chains/<chain_id>/messages/<message_hash>")]
pub async fn route(
    context: RequestContext,
    chain_id: String,
    message_hash: String,
) -> ApiResult<content::RawJson<String>> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);

    // Request
    let url = core_uri!(info_provider, "/v1/messages/{}/", message_hash)?;
    let http_request = Request::new(url);
    let body = info_provider.client().get(http_request).await?.body;
    let backend_message: Message = serde_json::from_str::<Message>(&body)?;

    // Request Safe Info with the safe field that was retrieved from the Message
    let safe_info = info_provider.safe_info(&backend_message.safe).await?;

    let message: FrontendMessage = map_message(&info_provider, &safe_info, &backend_message).await;

    let body = serde_json::to_string(&message)?;
    return Ok(content::RawJson(body));
}
