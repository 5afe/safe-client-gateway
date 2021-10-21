use crate::common::models::data_decoded::DataDecoded;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::contracts::models::DataDecoderRequest;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::http_client::Request;
use serde_json::json;

pub async fn request_data_decoded(
    context: &RequestContext,
    chain_id: &str,
    data_decoder_request: &DataDecoderRequest,
) -> ApiResult<DataDecoded> {
    let info_provider = DefaultInfoProvider::new(chain_id, context);
    let client = context.http_client();
    let url = core_uri!(info_provider, "/v1/data-decoder/")?;
    let body = json!({"data": &data_decoder_request.data});

    let request = {
        let mut request = Request::new(url);
        request.body(Some(body.to_string()));
        request
    };

    let response_body = client.post(request).await?.body;
    Ok(serde_json::from_str::<DataDecoded>(&response_body)?)
}
