use crate::utils::context::*;

#[rocket::async_test]
async fn testing_mocked_http_client() {
    // let mut mock_info_provider = MockInfoProvider::new();
    // mock_info_provider
    //     .expect_address_ex_from_contracts()
    //     .times(1)
    //     .return_once(move |_| {
    //         Ok(AddressEx {
    //             value: "0xb6029EA3B2c51D09a50B53CA8012FeEB05bDa35A".to_string(),
    //             name: Some("Address name".to_string()),
    //             logo_uri: Some("logo.url".to_string()),
    //         })
    //     });

    let response_json = "{\"valid\":\"json\"}";

    let mut mock_info_provider = MockHttpClient::new();
    mock_info_provider
        .expect_get()
        .times(1)
        .return_once(move |_| Ok(String::from(response_json)));

    let request_context = RequestContext::mock("request_id".to_string(), mock_info_provider);
    let actual = request_context
        .http_client
        .get("https://example.com")
        .await
        .expect("response error");
    assert_eq!(response_json, actual);
}
