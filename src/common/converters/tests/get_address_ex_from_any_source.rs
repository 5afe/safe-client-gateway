use crate::common::converters::get_address_ex_from_any_source;
use crate::common::models::addresses::AddressEx;
use crate::providers::info::*;

#[rocket::async_test]
async fn get_address_info_address_diff_than_safe() {
    let address = "0x1234";
    let safe = "0x4321";

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| {
            Ok(AddressEx {
                value: address.to_string(),
                name: Some("".to_string()),
                logo_uri: None,
            })
        });

    let expected = AddressEx {
        value: address.to_string(),
        name: Some("".to_string()),
        logo_uri: None,
    };

    let actual = get_address_ex_from_any_source(safe, address, &mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn get_address_info_address_diff_than_safe_error() {
    let address = "0x1234";
    let safe = "0x4321";

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let actual = get_address_ex_from_any_source(safe, address, &mut mock_info_provider).await;
    assert_eq!(AddressEx::address_only(address), actual);
}

#[rocket::async_test]
async fn get_address_info_address_equal_to_safe() {
    let address = "0x1234";
    let safe = "0x1234";

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(0);

    let actual = get_address_ex_from_any_source(safe, address, &mut mock_info_provider).await;
    assert_eq!(AddressEx::address_only(address), actual);
}
