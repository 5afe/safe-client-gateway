mod test {
    use crate::utils::json::default_if_null;
    use serde::Deserialize;


    #[derive(PartialEq, Deserialize, Debug)]
    struct ExpectedStruct {
        #[serde(deserialize_with = "default_if_null")]
        field: u64,
        other_field: String,
        #[serde(deserialize_with = "default_if_null")]
        with_custom_default: WithCustomDefault,
    }

    #[derive(PartialEq, Deserialize, Debug)]
    struct WithCustomDefault(String);

    impl Default for WithCustomDefault {
        fn default() -> Self {
            WithCustomDefault("custom default".to_string())
        }
    }

    #[test]
    #[should_panic]
    fn deserialize_missing_expected_field() {
        let test_json = json!({
        "other_field":"other value"
    });

        serde_json::from_str::<ExpectedStruct>(&test_json.to_string()).unwrap();
    }

    #[test]
    fn deserialize_expected_field_null() {
        let test_json = json!({
        "field" : null,
        "other_field":"other value",
        "with_custom_default": "different value"
    });

        let expected = ExpectedStruct {
            field: 0,
            other_field: "other value".to_string(),
            with_custom_default: WithCustomDefault("different value".to_string()),
        };

        let actual = serde_json::from_str::<ExpectedStruct>(&test_json.to_string()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_expected_field_with_value() {
        let test_json = json!({
        "field" : 42,
        "other_field":"other value",
        "with_custom_default": "different value"
    });

        let expected = ExpectedStruct {
            field: 42,
            other_field: "other value".to_string(),
            with_custom_default: WithCustomDefault("different value".to_string()),
        };

        let actual = serde_json::from_str::<ExpectedStruct>(&test_json.to_string()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_expected_field_with_custom_default() {
        let test_json = json!({
        "field" : 42,
        "other_field":"other value",
        "with_custom_default": null
    });

        let expected = ExpectedStruct {
            field: 42,
            other_field: "other value".to_string(),
            with_custom_default: WithCustomDefault("custom default".to_string()),
        };

        let actual = serde_json::from_str::<ExpectedStruct>(&test_json.to_string()).unwrap();

        assert_eq!(expected, actual);
    }
}