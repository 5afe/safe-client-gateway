use crate::common::models::data_decoded::{
    DataDecoded, InternalTransaction, Operation, ParamValue, Parameter, ValueDecodedType,
};
use crate::tests::json;

#[test]
fn deserialise_params_value_as_string() {
    let json = r#"
    {
        "name": "_threshold",
        "type": "uint256",
        "value": "2"
    }
    "#;

    let actual = serde_json::from_str::<Parameter>(json);

    let expected = Parameter {
        name: "_threshold".to_string(),
        param_type: "uint256".to_string(),
        value: "2".to_string().into(),
        value_decoded: None,
    };

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn deserialise_params_value_as_array() {
    let json = r#"
    {
        "name": "_owners",
        "type": "address[]",
        "value": [
            "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0",
            "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd"
        ]
    }
    "#;

    let actual = serde_json::from_str::<Parameter>(json);

    let expected = Parameter {
        name: "_owners".to_string(),
        param_type: "address[]".to_string(),
        value_decoded: None,
        value: ParamValue::ArrayValue(vec![
            "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0"
                .to_string()
                .into(),
            "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd"
                .to_string()
                .into(),
        ]),
    };

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn deserialize_decoded_value() {
    let actual = serde_json::from_str::<DataDecoded>(json::DATA_DECODED_MULTI_SEND).unwrap();

    let expected = DataDecoded {
        method: "multiSend".to_string(),
        parameters: Some(vec![
            Parameter {
                name: String::from("transactions"),
                param_type: String::from("bytes"),
                value: ParamValue::SingleValue(String::from("0x00d9ba894e0097f8cc2bbc9d24d308b98e36dc6d0200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000044a9059cbb000000000000000000000000938bae50a210b80ea233112800cd5bc2e764430000000000000000000000000000000000000000000000000000038d7ea4c6800000d9ba894e0097f8cc2bbc9d24d308b98e36dc6d0200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000044a9059cbb000000000000000000000000938bae50a210b80ea233112800cd5bc2e764430000000000000000000000000000000000000000000000000000038d7ea4c6800000d9ba894e0097f8cc2bbc9d24d308b98e36dc6d0200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000044a9059cbb000000000000000000000000938bae50a210b80ea233112800cd5bc2e764430000000000000000000000000000000000000000000000000000038d7ea4c68000")),
                value_decoded: Some(ValueDecodedType::InternalTransaction(vec![
                    InternalTransaction {
                        operation: Operation::CALL,
                        to: String::from("0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02"),
                        value: Some(0.to_string()),
                        data: Some(String::from("0xa9059cbb000000000000000000000000938bae50a210b80ea233112800cd5bc2e764430000000000000000000000000000000000000000000000000000038d7ea4c68000")),
                        data_decoded: Some(DataDecoded {
                            method: String::from("transfer"),
                            parameters: Some(vec![
                                Parameter {
                                    name: String::from("to"),
                                    param_type: String::from("address"),
                                    value: ParamValue::SingleValue(String::from("0x938bae50a210b80EA233112800Cd5Bc2e7644300")),
                                    value_decoded: None,
                                },
                                Parameter {
                                    name: String::from("value"),
                                    param_type: String::from("uint256"),
                                    value: ParamValue::SingleValue(String::from("1000000000000000")),
                                    value_decoded: None,
                                },
                            ]),
                        }),
                    },
                    InternalTransaction {
                        operation: Operation::CALL,
                        to: String::from("0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02"),
                        value: Some(0.to_string()),
                        data: Some(String::from("0xa9059cbb000000000000000000000000938bae50a210b80ea233112800cd5bc2e764430000000000000000000000000000000000000000000000000000038d7ea4c68000")),
                        data_decoded: Some(DataDecoded {
                            method: String::from("transfer"),
                            parameters: Some(vec![
                                Parameter {
                                    name: String::from("to"),
                                    param_type: String::from("address"),
                                    value: ParamValue::SingleValue(String::from("0x938bae50a210b80EA233112800Cd5Bc2e7644300")),
                                    value_decoded: None,
                                },
                                Parameter {
                                    name: String::from("value"),
                                    param_type: String::from("uint256"),
                                    value: ParamValue::SingleValue(String::from("1000000000000000")),
                                    value_decoded: None,
                                },
                            ]),
                        }),
                    },
                    InternalTransaction {
                        operation: Operation::CALL,
                        to: String::from("0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02"),
                        value: Some(0.to_string()),
                        data: Some(String::from("0xa9059cbb000000000000000000000000938bae50a210b80ea233112800cd5bc2e764430000000000000000000000000000000000000000000000000000038d7ea4c68000")),
                        data_decoded: Some(DataDecoded {
                            method: String::from("transfer"),
                            parameters: Some(vec![
                                Parameter {
                                    name: String::from("to"),
                                    param_type: String::from("address"),
                                    value: ParamValue::SingleValue(String::from("0x938bae50a210b80EA233112800Cd5Bc2e7644300")),
                                    value_decoded: None,
                                },
                                Parameter {
                                    name: String::from("value"),
                                    param_type: String::from("uint256"),
                                    value: ParamValue::SingleValue(String::from("1000000000000000")),
                                    value_decoded: None,
                                },
                            ]),
                        }),
                    }
                ])),
            },
        ]),
    };

    assert_eq!(actual, expected);
}
