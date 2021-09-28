use crate::services::safe_apps::safe_apps;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/v1/chains/<chain_id>/safe-apps")]
pub async fn get_safe_apps(
    context: Context<'_>,
    chain_id: String,
) -> ApiResult<content::Json<String>> {
    Ok(content::Json(serde_json::to_string(
        &safe_apps(&context, &chain_id).await?,
    )?))
}
