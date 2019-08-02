use serde::{de::DeserializeOwned, Serialize};

use crate::error::Result;

pub fn serialize<T: Serialize>(value: T) -> serde_json::Value {
    serde_json::to_value(value).expect("Types never fail to serialize.")
}

pub fn deserialize<T: DeserializeOwned>(value: serde_json::Value) -> Result<T> {
    serde_json::from_value(value).map_err(Into::into)
}
