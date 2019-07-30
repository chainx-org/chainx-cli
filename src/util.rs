use parity_codec::{Compact, Encode};
use serde::de::DeserializeOwned;

use sr_primitives::generic::Era;
use substrate_primitives::blake2_256;
use substrate_primitives::crypto::Pair as TraitPair;
use substrate_primitives::ed25519::Pair;
use substrate_primitives::hexdisplay::HexDisplay;

use chainx_primitives::{Acceleration, AccountId, Hash, Index};

use crate::error::Result;

pub fn deserialize<T: DeserializeOwned>(value: serde_json::Value) -> Result<T> {
    serde_json::from_value(value).map_err(Into::into)
}

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

pub fn gen_extrinsic(
    seed: &RawSeed,
    index: Index,
    function: runtime::Call,
    era: Era,
    hash: Hash,
    acc: Acceleration,
) -> String {
    let signed: runtime::Address = seed.account_id().into();
    let pair = seed.pair();
    let payload = (
        Compact::<Index>::from(index),
        function.clone(),
        era,
        hash,
        Compact::<Acceleration>::from(acc),
    );
    let signature = payload.using_encoded(|data| {
        if data.len() > 256 {
            pair.sign(&blake2_256(data))
        } else {
            pair.sign(data)
        }
    });

    // 编码字段 1 元组(发送人，签名)，func | 签名：(index, func, era, hash, acceleration)
    let utx = runtime::UncheckedExtrinsic::new_signed(index, function, signed, signature, era, acc);
    let t: Vec<u8> = utx.encode();
    format!("0x{:}", HexDisplay::from(&t))
}
