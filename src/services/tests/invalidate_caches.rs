use crate::models::backend::webhooks::{Payload, PayloadDetails, NewConfirmation, ExecutedMultisigTransaction, PendingMultisigTransaction};
use crate::services::hooks::invalidate_caches;
use mockall::predicate::*;
use crate::utils::cache::*;
use crate::utils::context::ContextCache;
use mockall::Sequence;

struct TestContext(MockCache);

impl ContextCache for TestContext {
    type Cache = MockCache;

    fn cache(&self) -> &Self::Cache {
        &self.0
    }
}

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
        .expect_invalidate()
        .times(0);
    mock_cache
        .expect_invalidate_pattern()
        .with(eq(String::from("*0x1230B3d59858296A31053C1b8562Ecf89A2f888b*")))
        .return_const(())
        .times(1);

    let context = TestContext(mock_cache);

    invalidate_caches(&context, &payload).unwrap();
}

#[test]
fn invalidate_new_confirmation_payload() {
    let payload = Payload {
        address: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        details: Some(PayloadDetails::NewConfirmation(
            NewConfirmation {
                owner: "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(),
                safe_tx_hash: "0x65df8a1e5a40703d9c67d5df6f9b552d3830faf0507c3d7350ba3764d3a68621".to_string(),
            })),
    };

    let mut mock_cache = MockCache::new();
    let mut sequence = Sequence::new();
    mock_cache
        .expect_fetch()
        .times(0);
    mock_cache
        .expect_create()
        .times(0);
    mock_cache
        .expect_invalidate()
        .times(0);
    mock_cache
        .expect_invalidate_pattern()
        .times(1)
        .return_const(())
        .with(eq(String::from("*0x1230B3d59858296A31053C1b8562Ecf89A2f888b*")))
        .in_sequence(&mut sequence);
    mock_cache
        .expect_invalidate_pattern()
        .times(1)
        .return_const(())
        .with(eq(String::from("*0x65df8a1e5a40703d9c67d5df6f9b552d3830faf0507c3d7350ba3764d3a68621*")))
        .in_sequence(&mut sequence);

    let context = TestContext(mock_cache);

    invalidate_caches(&context, &payload).unwrap();
}

#[test]
fn invalidate_executed_multisig_transaction_payload() {
    let payload = Payload {
        address: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        details: Some(PayloadDetails::ExecutedMultisigTransaction(
            ExecutedMultisigTransaction {
                safe_tx_hash: "0x65df8a1e5a40703d9c67d5df6f9b552d3830faf0507c3d7350ba3764d3a68621".to_string(),
                tx_hash: "0x0ebb2c317f55c96469e0ed2014f5833dc02a70b42f0ac52f4630938900caa698".to_string(),
            })),
    };

    let mut mock_cache = MockCache::new();
    let mut sequence = Sequence::new();
    mock_cache
        .expect_fetch()
        .times(0);
    mock_cache
        .expect_create()
        .times(0);
    mock_cache
        .expect_invalidate()
        .times(0);
    mock_cache
        .expect_invalidate_pattern()
        .times(1)
        .return_const(())
        .with(eq(String::from("*0x1230B3d59858296A31053C1b8562Ecf89A2f888b*")))
        .in_sequence(&mut sequence);
    mock_cache
        .expect_invalidate_pattern()
        .times(1)
        .return_const(())
        .with(eq(String::from("*0x65df8a1e5a40703d9c67d5df6f9b552d3830faf0507c3d7350ba3764d3a68621*")))
        .in_sequence(&mut sequence);

    let context = TestContext(mock_cache);

    invalidate_caches(&context, &payload).unwrap();
}

#[test]
fn invalidate_pending_multisig_transaction_payload() {
    let payload = Payload {
        address: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        details: Some(PayloadDetails::PendingMultisigTransaction(
            PendingMultisigTransaction {
                safe_tx_hash: "0x65df8a1e5a40703d9c67d5df6f9b552d3830faf0507c3d7350ba3764d3a68621".to_string(),
            })),
    };

    let mut mock_cache = MockCache::new();
    let mut sequence = Sequence::new();
    mock_cache
        .expect_fetch()
        .times(0);
    mock_cache
        .expect_create()
        .times(0);
    mock_cache
        .expect_invalidate()
        .times(0);
    mock_cache
        .expect_invalidate_pattern()
        .times(1)
        .return_const(())
        .with(eq(String::from("*0x1230B3d59858296A31053C1b8562Ecf89A2f888b*")))
        .in_sequence(&mut sequence);
    mock_cache
        .expect_invalidate_pattern()
        .times(1)
        .return_const(())
        .with(eq(String::from("*0x65df8a1e5a40703d9c67d5df6f9b552d3830faf0507c3d7350ba3764d3a68621*")))
        .in_sequence(&mut sequence);

    let context = TestContext(mock_cache);

    invalidate_caches(&context, &payload).unwrap();
}