use crate::models::commons::Parameter;

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

    let expected = Parameter::SingleValue {
        name: "_threshold".to_string(),
        param_type: "uint256".to_string(),
        value: "2".to_string(),
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

    let expected = Parameter::ArrayValue {
        name: "_owners".to_string(),
        param_type: "address[]".to_string(),
        value: vec![
            "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(),
            "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string()
        ],
    };

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}