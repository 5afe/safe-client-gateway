use crate::models::commons::{Parameter, ParamValue};

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
        value: ParamValue::ArrayValue(
            vec!(
                "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string().into(),
                "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string().into()
            )
        ),
    };

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}