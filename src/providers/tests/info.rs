use rocket::local::asynchronous::Client;

#[rocket::async_test]
async fn default_info_provider_chain_info() {}

#[rocket::async_test]
async fn default_info_provider_chain_info_in_mem_cache() {}

#[rocket::async_test]
async fn default_info_provider_chain_info_not_found() {}
