use crate::utils::urls::build_manifest_url;

#[test]
fn disallow_non_http_schemes() {
    let input_url = "ipfs://localhost";

    match build_manifest_url(input_url) {
        Err(error) => {
            assert_eq!(error.details.message.unwrap(), "Invalid scheme");
        }
        _ => {
            panic!("Test didn't error as expected")
        }
    };
}

#[test]
fn disallow_localhost() {
    let input_url = "https://localhost";

    match build_manifest_url(input_url) {
        Err(error) => {
            assert_eq!(error.details.message.unwrap(), "Localhost not accepted");
        }
        _ => {
            panic!("Test didn't error as expected")
        }
    };
}

#[test]
fn disallow_ip_address() {
    let input_url = "http://127.0.0.1";

    match build_manifest_url(input_url) {
        Err(error) => {
            assert_eq!(error.details.message.unwrap(), "IP address not accepted");
        }
        _ => {
            panic!("Test didn't error as expected")
        }
    };
}

#[test]
fn valid_url_no_trailing_slash() {
    let input_url = "https://happy.path";

    let actual = build_manifest_url(input_url).unwrap();
    assert_eq!(actual, "https://happy.path/manifest.json")
}

#[test]
fn valid_url_with_trailing_slash() {
    let input_url = "https://happy.path/";

    let actual = build_manifest_url(input_url).unwrap();
    assert_eq!(actual, "https://happy.path/manifest.json")
}

#[test]
fn valid_url_with_trailing_slash_and_port() {
    let input_url = "https://happy.path:8000/";

    let actual = build_manifest_url(input_url).unwrap();
    assert_eq!(actual, "https://happy.path:8000/manifest.json")
}

#[test]
fn valid_url_with_longer_path() {
    let input_url = "https://cloudflare-ipfs.com/ipfs/QmQs6CUbMUyKe3Sa3tU3HcnWWzsuCk8oJEk8CZKhRcJfEh";

    let actual = build_manifest_url(input_url).unwrap();
    assert_eq!(actual, "https://cloudflare-ipfs.com/ipfs/QmQs6CUbMUyKe3Sa3tU3HcnWWzsuCk8oJEk8CZKhRcJfEh/manifest.json")
}
