use crate::models::service::transactions::summary::SafeAppInfo;
use crate::providers::info::InfoProvider;
use serde::Deserialize;

pub fn to_safe_app_info(origin: &str, info_provider: &mut dyn InfoProvider) -> Option<SafeAppInfo> {
    let origin_internal = serde_json::from_str::<OriginInternal>(origin).ok();
    origin_internal
        .as_ref()
        .map(|origin| origin.to_safe_app_info(info_provider))
}

#[derive(Deserialize, Debug)]
struct OriginInternal {
    url: String,
}

impl OriginInternal {
    fn to_safe_app_info(&self, info_provider: &mut dyn InfoProvider) -> SafeAppInfo {
        let manifest_url = format!("{}/manifest.json", self.url);
        // info_provider.

        SafeAppInfo {
            name: "".to_string(),
            url: "".to_string(),
            logo_url: "".to_string(),
        }
    }
}
