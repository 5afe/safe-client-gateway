use crate::cache::cache_operations::CacheResponse;
use crate::services::safes::get_safe_info_ex;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/v1/safes/<safe_address>")]
pub fn safe_info(context: Context, safe_address: String) -> ApiResult<content::Json<String>> {
    CacheResponse::new()
        .key(context.uri())
        .resp_generator(|| get_safe_info_ex(&context, &safe_address))
        .execute(context.cache())
}
