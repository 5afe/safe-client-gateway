use crate::providers::info::{InfoProvider, SafeAppInfo};
use serde::Deserialize;

pub async fn safe_app_info_from(
    origin: &str,
    info_provider: &mut dyn InfoProvider,
) -> Option<SafeAppInfo> {
    let origin_internal = serde_json::from_str::<OriginInternal>(origin).ok();
    origin_internal.as_ref().and_then(|origin| {
        info_provider
            .safe_app_info(&origin.url.replace("ipfs.io", "cloudflare-ipfs.com"))
            .await
            .ok()
    })
}

#[derive(Deserialize, Debug, PartialEq)]
pub(super) struct OriginInternal {
    pub(super) url: String,
}
