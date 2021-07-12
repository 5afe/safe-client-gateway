use crate::monitoring::performance::extract_chain_id;

#[test]
fn chain_dependent_endpoint() {
    let uri = uri!("/v1/chains/1/safes/0x1230B3d59858296A31053C1b8562Ecf89A2f888b");
    let actual = extract_chain_id(&uri.path());

    assert_eq!("1", actual);
}

#[test]
fn chain_info_endpoint_single() {
    let uri = uri!("/v1/chains/1337");
    let actual = extract_chain_id(&uri.path());

    assert_eq!("1337", actual);
}

#[test]
fn chain_info_all() {
    let uri = uri!("/v1/chains");

    let actual = extract_chain_id(&uri.path());
    assert_eq!("-1", actual);
}

#[test]
fn chain_independent_endpoint() {
    let uri = uri!("/about/redis/");

    let actual = extract_chain_id(&uri.path());
    assert_eq!("-1", actual);
}
