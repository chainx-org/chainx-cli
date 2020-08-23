use serde::{de::DeserializeOwned, Serialize};

use crate::error::Result;

pub fn serialize<T: Serialize>(value: T) -> serde_json::Value {
    serde_json::to_value(value).expect("Types never fail to serialize.")
}

#[allow(unused)]
pub fn deserialize<T: DeserializeOwned>(value: serde_json::Value) -> Result<T> {
    serde_json::from_value(value).map_err(Into::into)
}

pub fn blake2_256_and_hex(key: &[u8]) -> String {
    let hash = substrate_primitives::blake2_256(key);
    format!("0x{:}", substrate_primitives::hexdisplay::HexDisplay::from(&hash))
}
