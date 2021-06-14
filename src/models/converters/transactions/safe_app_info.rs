use crate::providers::info::{InfoProvider, SafeAppInfo};
use serde::Deserialize;

pub async fn safe_app_info_from(
    origin: &str,
    info_provider: &impl InfoProvider,
    chain_id: &str,
) -> Option<SafeAppInfo> {
    let origin_internal = serde_json::from_str::<OriginInternal>(origin).ok()?;
    info_provider
        .safe_app_info(
            chain_id,
            &origin_internal
                .url
                .replace("ipfs.io", "cloudflare-ipfs.com"),
        )
        .await
        .ok()
}

#[derive(Deserialize, Debug, PartialEq)]
pub(super) struct OriginInternal {
    pub(super) url: String,
}
