use crate::utils::context::Context;
use anyhow::Result;

pub fn invalidate_caches(context: &Context, safe: &String) -> Result<()> {
    context.cache().invalidate_pattern(&format!("*{}*", safe));
    Ok(())
}
