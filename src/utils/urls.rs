use crate::utils::errors::ApiResult;

pub fn build_manifest_url(url: &str) -> ApiResult<String> {
    Ok(format!("{}/manifest.json", url))
}
