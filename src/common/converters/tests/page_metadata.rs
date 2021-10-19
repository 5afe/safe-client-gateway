use crate::common::models::PageMetadata;

#[test]
fn page_metadata_with_valid_non_zero_data() {
    let input = "limit=20&offset=20&queued=false";

    let actual = PageMetadata::from_cursor(input);
    let expected = PageMetadata {
        offset: 20,
        limit: 20,
    };
    assert_eq!(expected, actual);
}

#[test]
fn page_metadata_with_zeros() {
    let input = "limit=0&offset=0";

    let actual = PageMetadata::from_cursor(input);
    let expected = PageMetadata {
        offset: 0,
        limit: 0,
    };
    assert_eq!(expected, actual);
}

#[test]
fn page_metadata_with_missing_optional_args() {
    let input = "offset=50";

    let actual = PageMetadata::from_cursor(input);
    let expected = PageMetadata {
        offset: 50,
        limit: 20,
    };
    assert_eq!(expected, actual);
}
