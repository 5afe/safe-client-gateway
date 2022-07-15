use rocket::futures::future::OptionFuture;
use serde::{Deserialize, Serialize};

use crate::common::models::addresses::AddressEx;
use crate::common::models::backend::transactions::SafeTransaction;
use crate::common::models::data_decoded::{DataDecoded, Operation};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::contracts::models::DataDecoderRequest;
use crate::routes::transactions::converters::details::is_trusted_delegate_call;
use crate::routes::transactions::models::details::TransactionData;
use crate::routes::transactions::models::TransactionInfo;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::http_client::Request;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionPreviewRequest {
    to: String,
    data: Option<String>,
    value: String,
    operation: Operation,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(Deserialize))]
pub struct TransactionPreview {
    pub tx_info: TransactionInfo,
    pub tx_data: TransactionData,
}

pub async fn preview_transaction(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    preview_request: &TransactionPreviewRequest,
) -> ApiResult<TransactionPreview> {
    let info_provider = DefaultInfoProvider::new(chain_id, context);

    // Get [AddressEx] for the [to] field
    let to_address_ex = info_provider
        .address_ex_from_contracts(&preview_request.to)
        .await
        .unwrap_or(AddressEx {
            value: preview_request.to.to_string(),
            name: None,
            logo_uri: None,
        });

    // Get [DataDecoded]
    let data_decoded: Option<DataDecoded> = match &preview_request.data {
        None => None,
        Some(data) => match decode_data(context, &info_provider, data).await {
            Err(_) => None,
            Ok(decoded_data) => Some(decoded_data),
        },
    };

    // We create a [SafeTransaction] so we can get the info out of it
    let safe_transaction = SafeTransaction {
        safe: safe_address.to_string(),
        to: preview_request.to.to_string(),
        value: Some(preview_request.value.to_string()),
        data: preview_request.data.clone(),
        data_decoded: data_decoded.clone(),
        operation: preview_request.operation,
    };
    let transaction_info: TransactionInfo = safe_transaction
        .transaction_info(&info_provider, false)
        .await;

    let is_trusted_delegate_call: Option<bool> = is_trusted_delegate_call(
        &safe_transaction.operation,
        &safe_transaction.to,
        &safe_transaction.data_decoded,
        &info_provider,
    )
    .await
    .unwrap_or(None);

    let transaction_data =
        TransactionData {
            hex_data: preview_request.data.clone(),
            data_decoded,
            to: to_address_ex,
            value: Some(preview_request.value.to_string()),
            operation: preview_request.operation,
            trusted_delegate_call_target: is_trusted_delegate_call,
            address_info_index: OptionFuture::from(safe_transaction.data_decoded.as_ref().map(
                |data_decoded| async move {
                    data_decoded.build_address_info_index(&info_provider).await
                },
            ))
            .await
            .flatten(),
        };

    Ok(TransactionPreview {
        tx_info: transaction_info,
        tx_data: transaction_data,
    })
}

async fn decode_data(
    context: &RequestContext,
    info_provider: &(impl InfoProvider + Sync),
    data: &str,
) -> ApiResult<DataDecoded> {
    let data_decoder_endpoint = core_uri!(info_provider, "/v1/data-decoder/")?;
    let client = context.http_client();

    let request = {
        let mut request = Request::new(data_decoder_endpoint);
        let body = DataDecoderRequest {
            data: data.to_string(),
        };
        request.body(Some(serde_json::to_string(&body)?));
        request
    };
    let response_body = client.post(request).await?.body;
    Ok(serde_json::from_str::<DataDecoded>(&response_body)?)
}
