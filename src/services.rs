pub mod transactions;

//TODO set network based in the url in which the API is exposed somehow?
// const SUPPORTED_NETWORKS: [&str; 2] = ["rinkeby", "mainnet"];

fn base_transaction_service_url() -> String {
    //TODO implement logic for selecting network mainnet/rinkeby
    String::from("https://safe-transaction.rinkeby.gnosis.io/api/v1")
}

fn base_4byte_service_url() -> String {
    String::from("https://www.4byte.directory/api/v1")
}
