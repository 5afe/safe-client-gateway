use serde::{Deserialize, Deserializer};

pub fn try_deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(T::deserialize(deserializer).ok())
}

pub fn default_if_null<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + serde::Deserialize<'de>,
{
    <Option<T> as serde::Deserialize>::deserialize(deserializer)
        .map(|result| result.unwrap_or_default())
}

pub fn remove_whitespace(input: &str) -> String {
    let mut expected = input.replace('\n', "");
    expected.retain(|c| !c.is_whitespace());
    expected
}
