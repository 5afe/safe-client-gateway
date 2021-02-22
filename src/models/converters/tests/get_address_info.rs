use crate::models::converters::get_address_info;
use crate::providers::address_info::AddressInfo;
use crate::providers::info::*;

#[test]
fn get_address_info_address_diff_than_safe() {
    let address = "0x1234";
    let safe = "0x4321";

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_address_info()
        .times(1)
        .return_once(move |_| {
            Ok(AddressInfo {
                name: "".to_string(),
                logo_uri: None,
            })
        });
    let expected = AddressInfo {
        name: "".to_string(),
        logo_uri: None,
    };

    let actual = get_address_info(safe, address, &mut mock_info_provider);

    assert!(actual.is_some());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn get_address_info_address_diff_than_safe_error() {
    let address = "0x1234";
    let safe = "0x4321";

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_address_info()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let actual = get_address_info(safe, address, &mut mock_info_provider);
    assert!(actual.is_none());
}

#[test]
fn get_address_info_address_equal_to_safe() {
    let address = "0x1234";
    let safe = "0x1234";

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_address_info().times(0);

    let actual = get_address_info(safe, address, &mut mock_info_provider);
    assert!(actual.is_none());
}
