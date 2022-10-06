use crate::routes::safe_apps::handlers::safe_apps;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use rocket::response::content;
use rocket_okapi::openapi;
/// `/v1/chains/<chain_id>/safe-apps` <br />
/// Returns [SafeApp](crate::routes::safe_apps::models::SafeApp)
///
/// # Safe Apps
///
/// This endpoint returns the list of Safe apps supported on a network
///
/// ## Path
///
/// - `/v1/chains/<chain_id>/safe-apps`
///
/// ## Query parameters
///
/// - `client_url`: The URL of the client application. Optional.
/// - `url`: Filter Safe Apps available from url. url needs to be an exact match. Optional.
///
/// ## Examples
///
/// [
///     {
///         "id": 24,
///         "url": "https://cloudflare-ipfs.com/ipfs/QmdVaZxDov4bVARScTLErQSRQoxgqtBad8anWuw3YPQHCs",
///         "name": "Transaction Builder",
///         "iconUrl": "https://cloudflare-ipfs.com/ipfs/QmdVaZxDov4bVARScTLErQSRQoxgqtBad8anWuw3YPQHCs/tx-builder.png",
///         "description": "A Safe app to compose custom transactions",
///         "chainIds": [
///             "1",
///             "4",
///             "56",
///             "100",
///             "137",
///             "246",
///             "73799"
///         ],
///         "provider": null,
///         "accessControl": {
///            "type": "NO_RESTRICTIONS"
///          }
///     },
///     {
///         "id": 25,
///         "url": "https://cloudflare-ipfs.com/ipfs/QmTpLhxSiD1H94BFxeV2P6RfJf6EyCxxUCVYpcDffyMmmZ",
///         "name": "WalletConnect",
///         "iconUrl": "https://cloudflare-ipfs.com/ipfs/QmTpLhxSiD1H94BFxeV2P6RfJf6EyCxxUCVYpcDffyMmmZ/wallet-connect.svg",
///         "description": "Allows your Gnosis Safe Multisig to connect to dapps via WalletConnect.",
///         "chainIds": [
///             "1",
///             "4",
///             "56",
///             "100",
///             "137",
///             "246",
///             "73799",
///             "42161"
///         ],
///         "provider": null,
///         "accessControl": {
///            "type": "DOMAIN_ALLOWLIST",
///           "value": ["https://gnosis-safe.io"]
///          }
///     }
/// ]
#[openapi(tag = "SafeApps")]
#[get("/v1/chains/<chain_id>/safe-apps?<client_url>&<url>")]
pub async fn get_safe_apps(
    context: RequestContext,
    chain_id: String,
    client_url: Option<String>,
    url: Option<String>,
) -> ApiResult<content::RawJson<String>> {
    Ok(content::RawJson(serde_json::to_string(
        &safe_apps(&context, &chain_id, &client_url, &url).await?,
    )?))
}
