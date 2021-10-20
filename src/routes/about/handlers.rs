extern crate reqwest;

use crate::config::{build_number, version};
use crate::providers::info::{DefaultInfoProvider, InfoProvider, NewDefaultInfoProvider};
use crate::routes::about::models::{About, ChainAbout};
use crate::routes::safes::models::Implementation;
use crate::utils::context::{Context, RequestContext};
use crate::utils::errors::ApiResult;

pub async fn chains_about(context: &RequestContext, chain_id: &str) -> ApiResult<ChainAbout> {
    let info_provider = NewDefaultInfoProvider::new_new(chain_id, &context);
    let chain_info = info_provider.chain_info().await?;
    let about = about();
    Ok(ChainAbout {
        transaction_service_base_uri: chain_info.transaction_service,
        about: About {
            name: about.name,
            version: about.version,
            build_number: about.build_number,
        },
    })
}

pub fn about() -> About {
    About {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: version(),
        build_number: build_number(),
    }
}

pub async fn get_master_copies(
    context: &RequestContext,
    chain_id: &str,
) -> ApiResult<Vec<Implementation>> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);
    Ok(info_provider
        .master_copies()
        .await?
        .into_iter()
        .map(|master_copy| master_copy.into())
        .collect())
}
