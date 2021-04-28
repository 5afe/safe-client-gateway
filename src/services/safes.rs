use crate::models::service::safes::{SafeInfoEx, SafeLastChanges, SafeState};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;

pub fn get_safe_info_ex(context: &Context, safe_address: &String) -> ApiResult<SafeState> {
    let mut info_provider = DefaultInfoProvider::new(context);
    let safe_info = info_provider.safe_info(safe_address)?;

    let safe_info_ex = safe_info.to_safe_info_ex(&mut info_provider);

    let safe_state = SafeState {
        safe_config: safe_info_ex,
        safe_state: SafeLastChanges {
            collectibles: None,
            tx_queued: None,
            tx_history: None,
        },
    };

    Ok(safe_state)
}
