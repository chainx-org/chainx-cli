use serde::{Deserialize, Serialize};

pub type Hash = substrate_primitives::H256;

#[derive(Clone, Serialize, Deserialize)]
pub struct EncodeWrapper(substrate_primitives::storage::StorageKey);

#[derive(Clone, Serialize, Deserialize)]
pub struct DecodeWrapper(substrate_primitives::storage::StorageData);

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Chain {
    ChainX,
    Bitcoin,
    Ethereum,
}

impl std::str::FromStr for Chain {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ChainX" | "PCX" => Ok(Chain::Bitcoin),
            "Bitcoin" | "BTC" => Ok(Chain::Bitcoin),
            "Ethereum" | "ETH" => Ok(Chain::Ethereum),
            _ => Err("Unknown Chain Type"),
        }
    }
}

#[derive(Debug)]
pub enum HashOrHeight {
    Height(u64),
    Hash(Hash),
}

impl std::str::FromStr for HashOrHeight {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") || s.starts_with("0X") {
            let hash = s.parse::<Hash>().map_err(|_| "Invalid Hash Length")?;
            return Ok(HashOrHeight::Hash(hash));
        }
        match s.parse::<u64>() {
            Ok(height) => Ok(HashOrHeight::Height(height)),
            Err(_) => Err("The param is neither a 0x-prefix hex hash nor a number"),
        }
    }
}
