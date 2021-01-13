use crate::models::service::transactions::summary::SafeAppInfo;
use crate::providers::info::InfoProvider;
use serde::Deserialize;

pub fn to_safe_app_info(origin: &str, info_provider: &mut dyn InfoProvider) -> Option<SafeAppInfo> {
    let origin_internal = serde_json::from_str::<OriginInternal>(origin).ok();
    origin_internal
        .as_ref()
        .and_then(|origin| origin.to_safe_app_info(info_provider))
}

#[derive(Deserialize, Debug, PartialEq)]
pub(super) struct OriginInternal {
    pub(super) url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub(super) struct Manifest {
    pub(super) name: String,
    pub(super) description: String,
    #[serde(rename(deserialize = "iconPath"))]
    pub(super) icon_path: String,
}

impl OriginInternal {
    fn to_safe_app_info(&self, info_provider: &mut dyn InfoProvider) -> Option<SafeAppInfo> {
        let manifest_url = format!("{}/manifest.json", self.url);
        info_provider
            .raw_request(&manifest_url)
            .ok()
            .and_then(|manifest_json| serde_json::from_str::<Manifest>(&manifest_json).ok())
            .map(|manifest| SafeAppInfo {
                name: manifest.name.to_owned(),
                url: self.url.clone(),
                logo_url: format!("{}/{}", self.url, manifest.icon_path),
            })
    }
}
