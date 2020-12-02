use crate::models::commons::PageMetadata;

#[test]
fn page_metadata_with_valid_non_zero_data() {
    let input = "limit=20&offset=20&queued=false";

    let actual = PageMetadata::from_url_string(input).unwrap();
    let expected = PageMetadata {
        offset: 20,
        limit: 20,
    };
    assert_eq!(expected, actual);
}

#[test]
fn page_metadata_with_zeros() {}

#[test]
fn page_metadata_with_missing_optional_args() {}
