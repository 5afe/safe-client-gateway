use crate::models::commons::DataDecoded;
use crate::models::service::utils::DataDecoderRequest;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use std::collections::HashMap;

pub async fn request_data_decoded(
    context: &Context<'_>,
    chain_id: &str,
    data_decoder_request: &DataDecoderRequest,
) -> ApiResult<DataDecoded> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);
    let url = core_uri!(info_provider, "/v1/data-decoder/")?;
    let mut json = HashMap::new();
    json.insert("data", &data_decoder_request.data);

    let response = context.client().post(url).json(&json).send().await?;

    Ok(serde_json::from_str::<DataDecoded>(
        &response.text().await?,
    )?)
}
