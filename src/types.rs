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
