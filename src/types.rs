use serde::{Deserialize, Serialize};

pub type AccountId = substrate_primitives::ed25519::Public;
pub type Hash = substrate_primitives::H256;
pub type TradingPairIndex = u32;

#[derive(Clone, Serialize, Deserialize)]
pub struct EncodeWrapper(substrate_primitives::storage::StorageKey);

#[derive(Clone, Serialize, Deserialize)]
pub struct DecodeWrapper(substrate_primitives::storage::StorageData);

pub struct Page {
    pub index: u32,
    pub size: u32,
}

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
pub enum HeightOrHash {
    Height(u64),
    Hash(String),
}

impl std::str::FromStr for HeightOrHash {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") || s.starts_with("0X") {
            return Ok(HeightOrHash::Hash(s.to_string()));
        }
        match s.parse::<u64>() {
            Ok(height) => Ok(HeightOrHash::Height(height)),
            Err(_) => Err("The param is neither a 0x-prefix hex hash nor a number"),
        }
    }
}
