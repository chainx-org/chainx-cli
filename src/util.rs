use parity_codec::{Compact, Encode};
use serde::{de::DeserializeOwned, Serialize};

use sr_primitives::generic::Era;
use substrate_primitives::blake2_256;
use substrate_primitives::crypto::Pair as TraitPair;
use substrate_primitives::hexdisplay::HexDisplay;

use chainx_primitives::{Acceleration, Hash, Index};

use crate::error::Result;
use crate::types::RawSeed;

pub fn serialize<T: Serialize>(value: T) -> serde_json::Value {
    serde_json::to_value(value).expect("Types never fail to serialize.")
}

pub fn deserialize<T: DeserializeOwned>(value: serde_json::Value) -> Result<T> {
    serde_json::from_value(value).map_err(Into::into)
}

#[allow(unused)]
pub fn gen_extrinsic(
    seed: &RawSeed,
    index: Index,
    function: runtime::Call,
    era: Era,
    hash: Hash,
    acc: Acceleration,
) -> String {
    let signed: runtime::Address = seed.account_id().into();
    let index = Compact::<Index>::from(index);
    let acc = Compact::<Acceleration>::from(acc);
    let pair = seed.pair();
    let payload = (index, function.clone(), era, hash, acc);
    let signature = payload.using_encoded(|data| {
        if data.len() > 256 {
            pair.sign(&blake2_256(data))
        } else {
            pair.sign(data)
        }
    });

    // 编码字段 1 元组(发送人，签名)，func | 签名：(index, func, era, hash, acceleration)
    let unchecked_extrinsic = runtime::UncheckedExtrinsic {
        signature: Some((signed, signature, index, era, acc)),
        function,
    };
    format!("0x{:}", HexDisplay::from(&unchecked_extrinsic.encode()))
}
