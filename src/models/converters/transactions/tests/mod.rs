use crate::models::converters::transactions::data_size;

mod data_size_calculation;
mod summary;
mod details;
mod transfer_type_checks;

#[test]
fn data_size_none() {
    let data: Option<String> = None;

    let actual = data_size(&data);
    let expected = 0;

    assert_eq!(expected, actual);
}

#[test]
fn data_size_0() {
    let data = Some(String::new());

    let actual = data_size(&data);
    let expected = 0;

    assert_eq!(expected, actual);
}

#[test]
fn data_size_0x() {
    let data = Some(String::from("0x"));

    let actual = data_size(&data);
    let expected = 0;

    assert_eq!(expected, actual);
}

#[test]
fn data_size_of_8_hex_chars() {
    let data = Some(String::from("0x12345678"));

    let actual = data_size(&data);
    let expected = 4;

    assert_eq!(expected, actual);
}