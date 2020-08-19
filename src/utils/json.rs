use serde::{Deserialize, Deserializer};

pub fn try_deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
{
    Ok(T::deserialize(deserializer).ok())
}