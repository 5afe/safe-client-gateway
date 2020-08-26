use crate::models::backend::webhooks::Payload;
use crate::services::hooks::invalidate;
use mockall::predicate::*;
use crate::utils::cache::*;

#[test]
fn invalidate_with_empty_payload() {
    let payload = Payload {
        address: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        details: None,
    };

    let mut mock_cache = MockCache::new();
    mock_cache
        .expect_fetch()
        .times(0);
    mock_cache
        .expect_create()
        .times(0);
    mock_cache
        .expect_invalidate_pattern()
        .with(eq(String::from("*0x1230B3d59858296A31053C1b8562Ecf89A2f888b*")))
        .return_const(());
    mock_cache
        .expect__invalidate()
        .times(0);

    invalidate(&payload, &mock_cache);
}
