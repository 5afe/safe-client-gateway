use crate::routes::transactions::filters::multisig::MultisigFilters;
use crate::routes::transactions::filters::QueryParam;

use super::module::ModuleFilters;
use super::transfer::TransferFilters;

#[test]
pub fn transfer_filters() {
    let filter_all_defined = TransferFilters {
        execution_date_gte: Some(String::from("1234")),
        execution_date_lte: Some(String::from("4321")),
        to: Some(String::from("0x1230B3d59858296A31053C1b8562Ecf89A2f888b")),
        value: Some(String::from("100")),
        token_address: Some(String::from("0x1230B3d59858296A31053C1b8562Ecf89A2f888b")),
    };

    let filter_none = TransferFilters {
        execution_date_gte: None,
        execution_date_lte: None,
        to: None,
        value: None,
        token_address: None,
    };

    let filter_only_to = TransferFilters {
        execution_date_gte: None,
        execution_date_lte: None,
        to: Some(String::from("0x1230B3d59858296A31053C1b8562Ecf89A2f888b")),
        value: None,
        token_address: None,
    };

    assert_eq!(
        filter_all_defined.as_query_param(),
        "execution_date__gte=1234&\
        execution_date__lte=4321&\
    to=0x1230B3d59858296A31053C1b8562Ecf89A2f888b&\
    value=100&\
    token_address=0x1230B3d59858296A31053C1b8562Ecf89A2f888b&"
    );
    assert_eq!(filter_none.as_query_param(), "");
    assert_eq!(
        filter_only_to.as_query_param(),
        "to=0x1230B3d59858296A31053C1b8562Ecf89A2f888b&"
    );
}

#[test]
pub fn module_filters() {
    let filter_all_defined = ModuleFilters {
        to: Some(String::from("0x1230B3d59858296A31053C1b8562Ecf89A2f888b")),
        module: Some(String::from("0x1230B3d59858296A31053C1b8562Ecf89A2f888b")),
    };

    let filter_none = ModuleFilters {
        to: None,
        module: None,
    };

    let filter_only_to = ModuleFilters {
        to: Some(String::from("0x1230B3d59858296A31053C1b8562Ecf89A2f888b")),
        module: None,
    };
    assert_eq!(
        filter_all_defined.as_query_param(),
        "to=0x1230B3d59858296A31053C1b8562Ecf89A2f888b&\
    module=0x1230B3d59858296A31053C1b8562Ecf89A2f888b&"
    );
    assert_eq!(filter_none.as_query_param(), "");
    assert_eq!(
        filter_only_to.as_query_param(),
        "to=0x1230B3d59858296A31053C1b8562Ecf89A2f888b&"
    );
}

#[test]
pub fn multisig_filters() {
    let filter_all_defined = MultisigFilters {
        execution_date_gte: Some(String::from("1234")),
        execution_date_lte: Some(String::from("4321")),
        to: Some(String::from("0x1230B3d59858296A31053C1b8562Ecf89A2f888b")),
        value: Some(String::from("100")),
        nonce: Some(String::from("50")),
        executed: Some(String::from("true")),
    };

    let filter_none = MultisigFilters {
        execution_date_gte: None,
        execution_date_lte: None,
        to: None,
        value: None,
        nonce: None,
        executed: None,
    };

    let filter_only_to = MultisigFilters {
        execution_date_gte: None,
        execution_date_lte: None,
        to: Some(String::from("0x1230B3d59858296A31053C1b8562Ecf89A2f888b")),
        value: None,
        nonce: None,
        executed: None,
    };

    assert_eq!(
        filter_all_defined.as_query_param(),
        "execution_date__gte=1234&\
        execution_date__lte=4321&\
    to=0x1230B3d59858296A31053C1b8562Ecf89A2f888b&\
    value=100&\
    nonce=50&\
    executed=true&"
    );
    assert_eq!(filter_none.as_query_param(), "");
    assert_eq!(
        filter_only_to.as_query_param(),
        "to=0x1230B3d59858296A31053C1b8562Ecf89A2f888b&"
    );
}
