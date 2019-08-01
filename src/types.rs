use serde::{Deserialize, Serialize};

use substrate_primitives::crypto::Pair as TraitPair;
use substrate_primitives::ed25519::Pair;

use chainx_primitives::AccountId;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Chain {
    ChainX,
    Bitcoin,
    Ethereum,
}

pub type TradingPairIndex = u32;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash, Debug)]
pub struct RawSeed(String);

impl RawSeed {
    pub fn new<S: Into<String>>(seed: S) -> Self {
        Self(seed.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    // Unsafe, for test only
    pub fn pair(&self) -> Pair {
        let seed = self.as_str();
        let mut s: [u8; 32] = [b' '; 32];
        let len = ::std::cmp::min(32, seed.len());
        s[..len].copy_from_slice(&seed.as_bytes()[..len]);
        Pair::from_seed(s)
    }

    pub fn account_id(&self) -> AccountId {
        let pair = Self::pair(self);
        AccountId::from_slice(pair.public().as_slice())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EncodeWrapper(substrate_primitives::storage::StorageKey);

#[derive(Clone, Serialize, Deserialize)]
pub struct DecodeWrapper(substrate_primitives::storage::StorageData);

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
