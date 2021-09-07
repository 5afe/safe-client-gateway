use crate::models::backend::safe_app::SafeApp as BackendSafeApp;
use crate::models::service::safe_app::{SafeApp, SafeAppProvider};

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
                .map(|chain_id| chain_id.into())
                .collect(),
            provider: safe_app
                .provider
                .into_iter()
                .map(|provider| SafeAppProvider {
                    url: provider.url.to_string(),
                    name: provider.name.to_string(),
                })
                .collect(),
        }
    }
}
