use std::env;

#[test]
fn config_uri_formats_correctly() {
    env::set_var("CONFIG_SERVICE_URL", "https://config-url-example.com");
    let expected = "https://config-url-example.com/api/example";

    let actual = config_uri!("/example");

    assert_eq!(expected, actual)
}

#[test]
fn config_uri_formats_correctly_with_substitution() {
    env::set_var("CONFIG_SERVICE_URL", "https://config-url-example.com");
    let expected = "https://config-url-example.com/api/example/safe";

    let actual = config_uri!("/example/{}", "safe");

    assert_eq!(expected, actual)
}
