mod chains;
mod routes;

pub(super) const BACKEND_CHAINS_INFO_PAGE: &str =
    include_str!("json/backend_chains_info_page.json");
pub(super) const EXPECTED_CHAINS_INFO_PAGE: &str =
    include_str!("json/expected_chains_info_page.json");
