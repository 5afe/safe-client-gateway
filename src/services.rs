pub mod transactions;

//TODO set network based in the url in which the API is exposed somehow?
// const SUPPORTED_NETWORKS: [&str; 2] = ["rinkeby", "mainnet"];

fn base_transaction_service_url() -> String {
    //TODO implement logic for selecting network mainnet/rinkeby
    // String::from("https://safe-transaction.rinkeby.gnosis.io/api/v1")
    String::from("https://safe-transaction.staging.gnosisdev.com/api/v1")
}
