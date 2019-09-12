use parity_codec::{Compact, Encode};
use serde_json::Value;

use chainx_primitives::{Acceleration, Index};
use sr_primitives::generic::Era;
use substrate_primitives::{blake2_256, hexdisplay::HexDisplay};

use crate::rpc::author::AuthorRpc;
use crate::transport::{BoxFuture, ChainXTransport};
use crate::types::{Hash, Token};

pub trait ChainXCall: AuthorRpc {
    /*
    fn transfer(&self, key: Hash, to: Hash, value: u64, token: Token, memo: Option<String>, acc: u32) -> BoxFuture<Value>;
    fn nominate(&self, key: Hash, to: Hash, value: u64, memo: Option<String>, acc: u32) -> BoxFuture<Value>;
    fn unnominate(&self, key: Hash, to: Hash, value: u64, memo: Option<String>, acc: u32) -> BoxFuture<Value>;
    fn refresh(&self, key: Hash, url: Option<Hash>, to_run: Option<bool>, next_key: Option<Hash>, about: Option<String>, acc: u32) -> BoxFuture<Value>;
    fn claim(&self, key: Hash, to: Hash, acc: u32) -> BoxFuture<Value>;
    fn register(&self, key: Hash, name: String) -> BoxFuture<Value>;
    fn deposit_claim(&self, key: Hash, token: Token, acc: u32) -> BoxFuture<Value>;
    fn check_vote_weight(&self, number: u32) -> BoxFuture<Value>;
    */
}

impl<T: web3::BatchTransport + 'static> ChainXCall for ChainXTransport<T> {
    /*
    fn transfer(&self, key: Hash, to: Hash, value: u64, token: Option<String>, memo: Option<String>, acc: u32) -> BoxFuture<Value> {

    }

    fn nominate(&self, key: Hash, to: Hash, value: u64, memo: Option<String>, acc: u32) -> BoxFuture<Value> {

    }

    fn unnominate(&self, key: Hash, to: Hash, value: u64, memo: Option<String>, acc: u32) -> BoxFuture<Value> {

    }

    fn refresh(&self, key: Hash, url: Option<Hash>, to_run: Option<bool>, next_key: Option<Hash>, about: Option<String>, acc: u32) -> BoxFuture<Value> {

    }

    fn claim(&self, key: Hash, to: Hash, acc: u32) -> BoxFuture<Value> {

    }

    fn register(&self, key: Hash, name: String) -> BoxFuture<Value> {

    }

    fn deposit_claim(&self, key: Hash, token: Token, acc: u32) -> BoxFuture<Value> {

    }

    fn check_vote_weight(&self, number: u32) -> BoxFuture<Value> {

    }
    */
}

/*
pub fn gen_extrinsic(
    seed: &RawSeed,
    index: Index,
    function: chainx_runtime::Call,
    era: Era,
    hash: Hash,
    acc: Acceleration,
) -> String {
    let signed: chainx_runtime::Address = seed.account_id().into();
    let index = Compact::<Index>::from(index);
    let acc = Compact::<Acceleration>::from(acc);
    let pair = seed.pair();
    let payload = (index, function.clone(), era, hash.into_inner(), acc);
    let signature = payload.using_encoded(|data| {
        if data.len() > 256 {
            pair.sign(&blake2_256(data))
        } else {
            pair.sign(data)
        }
    });

    // 编码字段 1 元组(发送人，签名)，func | 签名：(index, func, era, hash, acceleration)
    let unchecked_extrinsic = chainx_runtime::UncheckedExtrinsic {
        signature: Some((signed, signature, index, era, acc)),
        function,
    };
    format!("0x{:}", HexDisplay::from(&unchecked_extrinsic.encode()))
}
*/
