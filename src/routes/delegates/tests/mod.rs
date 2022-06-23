mod routes;

pub const BACKEND_LIST_DELEGATES_OF_SAFE: &str =
    include_str!("json/backend_list_delegates_of_safe.json");
pub const EXPECTED_LIST_DELEGATES_OF_SAFE: &str =
    include_str!("json/expected_list_delegates_of_safe.json");
pub const BACKEND_CREATE_DELEGATE: &str = include_str!("json/backend_create_delegate.json");
pub const BACKEND_DELETE_DELEGATE: &str = include_str!("json/backend_delete_delegate.json");
pub const BACKEND_DELETE_DELEGATE_SAFE: &str =
    include_str!("json/backend_delete_delegate_safe.json");
