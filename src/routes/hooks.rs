use crate::config::webhook_token;
use crate::models::backend::webhooks::Payload;
use crate::services::hooks::invalidate_caches;
use crate::utils::context::Context;
use anyhow::{bail, Result};
use rocket_contrib::json::Json;

#[post("/v1/hook/update/<token>", format = "json", data = "<update>")]
pub fn update(context: Context, token: String, update: Json<Payload>) -> Result<()> {
    if token != webhook_token() {
        bail!("Invalid token");
    }
    invalidate_caches(context.cache(), &update)
}
