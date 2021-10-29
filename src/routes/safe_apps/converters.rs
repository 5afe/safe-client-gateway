use crate::common::models::backend::safe_apps::SafeApp as BackendSafeApp;
use crate::routes::safe_apps::models::{SafeApp, SafeAppProvider};

impl From<BackendSafeApp> for SafeApp {
    fn from(safe_app: BackendSafeApp) -> Self {
        SafeApp {
            id: safe_app.id,
            url: safe_app.url.to_string(),
            name: safe_app.name.to_string(),
            icon_url: safe_app.icon_url.to_string(),
            description: safe_app.description.to_string(),
            chain_ids: safe_app
                .chain_ids
                .into_iter()
                .map(|chain_id| chain_id.to_string())
                .collect(),
            provider: safe_app.provider.as_ref().map(|provider| SafeAppProvider {
                url: provider.url.to_string(),
                name: provider.name.to_string(),
            }),
        }
    }
}
