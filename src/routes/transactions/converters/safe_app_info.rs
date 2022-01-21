use crate::providers::info::{InfoProvider, SafeAppInfo};
use serde::Deserialize;

pub async fn safe_app_info_from(
    origin: &str,
    info_provider: &(impl InfoProvider + Sync),
) -> Option<SafeAppInfo> {
    let origin_internal = serde_json::from_str::<OriginInternal>(origin).ok()?;
    log::error!("{:#?}", &origin_internal);
    info_provider
        .safe_app_info(
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
