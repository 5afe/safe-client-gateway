use crate::models::commons::Page;
use crate::models::backend::transactions::Transaction;

#[test]
fn can_parse_response_page_with_nested_safe_transaction() {
    serde_json::from_str::<Page<Transaction>>(crate::json::RESPONSES_NESTED_SAFE_INTERACTION).unwrap();
}