use crate::utils::context::Context;
use crate::models::backend::webhooks::Payload;
use crate::config::webhook_token;
use crate::services::hooks::invalidate_caches;
use rocket_contrib::json::{Json};
use anyhow::{bail, Result};

#[post("/v1/hook/update/<token>", format = "json", data = "<update>")]
pub fn update(context: Context, token: String, update: Json<Payload>) -> Result<()> {
    if token != webhook_token() {
        bail!("Invalid token");
    }
    println!("{:?}", update.details);
    invalidate_caches(&context, &update.address)
}