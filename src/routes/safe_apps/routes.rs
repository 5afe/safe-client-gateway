use crate::routes::safe_apps::handlers::safe_apps;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use rocket::response::content;

/**
 * `/v1/chains/<chain_id>/safe-apps` <br />
 * Returns [SafeApp](crate::routes::safe_apps::models::SafeApp)
 *
 * # Safe Apps
 *
 * This endpoint returns the list of Safe apps supported on a network
 *
 * ## Path
 *
 * - `/v1/chains/<chain_id>/safe-apps
 *
 * ## Examples
 * [
 *     {
 *         "id": 24,
 *         "url": "https://cloudflare-ipfs.com/ipfs/QmdVaZxDov4bVARScTLErQSRQoxgqtBad8anWuw3YPQHCs",
 *         "name": "Transaction Builder",
 *         "iconUrl": "https://cloudflare-ipfs.com/ipfs/QmdVaZxDov4bVARScTLErQSRQoxgqtBad8anWuw3YPQHCs/tx-builder.png",
 *         "description": "A Safe app to compose custom transactions",
 *         "chainIds": [
 *             "1",
 *             "4",
 *             "56",
 *             "100",
 *             "137",
 *             "246",
 *             "73799"
 *         ],
 *         "provider": null
 *     },
 *     {
 *         "id": 25,
 *         "url": "https://cloudflare-ipfs.com/ipfs/QmTpLhxSiD1H94BFxeV2P6RfJf6EyCxxUCVYpcDffyMmmZ",
 *         "name": "WalletConnect",
 *         "iconUrl": "https://cloudflare-ipfs.com/ipfs/QmTpLhxSiD1H94BFxeV2P6RfJf6EyCxxUCVYpcDffyMmmZ/wallet-connect.svg",
 *         "description": "Allows your Gnosis Safe Multisig to connect to dapps via WalletConnect.",
 *         "chainIds": [
 *             "1",
 *             "4",
 *             "56",
 *             "100",
 *             "137",
 *             "246",
 *             "73799",
 *             "42161"
 *         ],
 *         "provider": null
 *     }
 * ]
 */
#[get("/v1/chains/<chain_id>/safe-apps")]
pub async fn get_safe_apps(
    context: RequestContext,
    chain_id: String,
) -> ApiResult<content::Json<String>> {
    Ok(content::Json(serde_json::to_string(
        &safe_apps(&context, &chain_id).await?,
    )?))
}
