use crate::common::models::data_decoded::ParamValue::SingleValue;
use crate::common::models::data_decoded::{DataDecoded, Parameter};
use crate::routes::transactions::converters::check_sender_or_receiver;

#[test]
fn check_sender_or_receiver_safe_sender() {
    let data_decoded = Some(DataDecoded {
        method: "transfer".to_string(),
        parameters: Some(vec![Parameter {
            name: "from".to_string(),
            param_type: "address".to_string(),
            value: SingleValue("0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string()),
            value_decoded: None,
        }]),
    });

    let actual =
        check_sender_or_receiver(&data_decoded, "0x1230B3d59858296A31053C1b8562Ecf89A2f888b");
    assert!(actual);
}

#[test]
fn check_sender_or_receiver_safe_receiver() {
    let data_decoded = Some(DataDecoded {
        method: "transfer".to_string(),
        parameters: Some(vec![Parameter {
            name: "to".to_string(),
            param_type: "address".to_string(),
            value: SingleValue("0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string()),
            value_decoded: None,
        }]),
    });

    let actual =
        check_sender_or_receiver(&data_decoded, "0x1230B3d59858296A31053C1b8562Ecf89A2f888b");
    assert!(actual);
}

#[test]
fn check_sender_or_receiver_safe_invalid_sender() {
    let data_decoded = Some(DataDecoded {
        method: "transfer".to_string(),
        parameters: Some(vec![Parameter {
            name: "from".to_string(),
            param_type: "address".to_string(),
            value: SingleValue("0x2230B3d59858296A31053C1b8562Ecf89A2f888b".to_string()),
            value_decoded: None,
        }]),
    });

    let actual =
        check_sender_or_receiver(&data_decoded, "0x1230B3d59858296A31053C1b8562Ecf89A2f888b");
    assert!(actual);
}

#[test]
fn check_sender_or_receiver_safe_invalid_receiver() {
    let data_decoded = Some(DataDecoded {
        method: "transfer".to_string(),
        parameters: Some(vec![Parameter {
            name: "to".to_string(),
            param_type: "address".to_string(),
            value: SingleValue("021230B3d59858296A31053C1b8562Ecf89A2f888b".to_string()),
            value_decoded: None,
        }]),
    });

    let actual =
        check_sender_or_receiver(&data_decoded, "0x1230B3d59858296A31053C1b8562Ecf89A2f888b");
    assert!(actual);
}

#[test]
fn check_sender_or_receiver_data_decoded_none() {
    let data_decoded: Option<DataDecoded> = Option::None;

    let actual = check_sender_or_receiver(&data_decoded, "");

    assert!(!actual);
}

#[test]
fn check_sender_or_receiver_everything_wrong() {
    let data_decoded = Some(DataDecoded {
        method: "wrong_transfer_method".to_string(),
        parameters: Some(vec![Parameter {
            name: "to".to_string(),
            param_type: "address".to_string(),
            value: SingleValue("0x2230B3d59858296A31053C1b8562Ecf89A2f888b".to_string()),
            value_decoded: None,
        }]),
    });

    let actual =
        check_sender_or_receiver(&data_decoded, "0x1230B3d59858296A31053C1b8562Ecf89A2f888b");
    assert!(!actual);
}
