use crate::models::service::safes::SafeInfoEx;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;
use rocket_contrib::json::JsonValue;

#[get("/v1/safes/<safe_address>")]
pub fn safe_info(context: Context, safe_address: String) -> ApiResult<JsonValue> {
    Ok(json!({
         "status": "new",
         "reason": "Resource in construction."
    }))
}
