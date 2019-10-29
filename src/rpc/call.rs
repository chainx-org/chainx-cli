use codec::{Compact, Encode};
use serde_json::Value;
use web3::futures::Future;

use chainx_primitives::{Acceleration, AccountId, Index};
use chainx_runtime::{Call as RuntimeCall, Runtime};
use sr_primitives::generic::Era;
use substrate_primitives::blake2_256;
use substrate_primitives::ed25519::{self, Pair};
use substrate_primitives::hexdisplay::HexDisplay;
use substrate_primitives::Pair as TraitPair;

use crate::rpc::author::AuthorRpc;
use crate::rpc::chain::ChainRpc;
use crate::rpc::state::storage::StorageRpc;
use crate::transport::{BoxFuture, ChainXTransport};
use crate::types::{Hash, Token};
use crate::util;

pub trait ChainXCall: AuthorRpc + ChainRpc + StorageRpc {
    fn submit_call(&self, pair: Pair, func: RuntimeCall, acc: u32) -> BoxFuture<Value>;
    fn transfer(&self, key: Hash, to: Hash, value: u64, token: Token, memo: String, acc: u32) -> BoxFuture<Value>;
    fn multisig_propose(&self, key: Hash, multisig_addr: Hash, proposal: RuntimeCall, acc: u32) -> BoxFuture<Value>;
    fn multisig_confirm(&self, key: Hash, multisig_addr: Hash, id: Hash, acc: u32) -> BoxFuture<Value>;
    fn multisig_remove(&self, key: Hash, multisig_addr: Hash, id: Hash, acc: u32) -> BoxFuture<Value>;
    fn nominate(&self, key: Hash, target: Hash, value: u64, memo: String, acc: u32) -> BoxFuture<Value>;
    fn unnominate(&self, key: Hash, target: Hash, value: u64, memo: String, acc: u32) -> BoxFuture<Value>;
    fn renominate(&self, key: Hash, from: Hash, to: Hash, value: u64, memo: String, acc: u32) -> BoxFuture<Value>;
    fn unfreeze(&self, key: Hash, target: Hash, index: u32, acc: u32) -> BoxFuture<Value>;
    fn vote_claim(&self, key: Hash, target: Hash, acc: u32) -> BoxFuture<Value>;
    fn deposit_claim(&self, key: Hash, token: Token, acc: u32) -> BoxFuture<Value>;
}

impl<T: web3::BatchTransport + 'static> ChainXCall for ChainXTransport<T> {
    fn submit_call(&self, pair: Pair, func: RuntimeCall, acc: u32) -> BoxFuture<Value> {
        let sender = Hash::from(pair.public().0);
        Box::new(
            self.block_hash(Some(0))
                .and_then(util::deserialize::<Hash>)
                .join(self.account_nonce(sender, None))
                .and_then(move |(hash, nonce)| {
                    let key = LocalKey::Ed25519(pair);
                    let era = Era::Immortal;
                    let extrinsic = gen_extrinsic(key, nonce, func, era, hash, acc);
                    self.submit_extrinsic(&extrinsic)
                }),
        )
    }

    fn transfer(&self, key: Hash, to: Hash, value: u64, token: Token, memo: String, acc: u32) -> BoxFuture<Value> {
        let pair = Pair::from_seed(key.into_inner().as_fixed_bytes());
        let to: chainx_runtime::Address = AccountId::from_h256(to.into_inner()).into();
        let token = token.name();
        let memo = memo.into_bytes();
        let func = RuntimeCall::XAssets(xassets::Call::transfer::<Runtime>(to, token, value, memo));
        self.submit_call(pair, func, acc)
    }

    fn multisig_propose(&self, key: Hash, multisig_addr: Hash, proposal: RuntimeCall, acc: u32) -> BoxFuture<Value> {
        let pair = Pair::from_seed(key.into_inner().as_fixed_bytes());
        let multisig = AccountId::from_h256(multisig_addr.into_inner());
        let proposal = Box::new(proposal);
        let func = RuntimeCall::XMultiSig(xmultisig::Call::execute::<Runtime>(multisig, proposal));
        self.submit_call(pair, func, acc)
    }

    fn multisig_confirm(&self, key: Hash, multisig_addr: Hash, id: Hash, acc: u32) -> BoxFuture<Value> {
        let pair = Pair::from_seed(key.into_inner().as_fixed_bytes());
        let multisig = AccountId::from_h256(multisig_addr.into_inner());
        let id = id.into_inner();
        let func = RuntimeCall::XMultiSig(xmultisig::Call::confirm::<Runtime>(multisig, id));
        self.submit_call(pair, func, acc)
    }

    fn multisig_remove(&self, key: Hash, multisig_addr: Hash, id: Hash, acc: u32) -> BoxFuture<Value> {
        let pair = Pair::from_seed(key.into_inner().as_fixed_bytes());
        let multisig = AccountId::from_h256(multisig_addr.into_inner());
        let id = id.into_inner();
        let func = RuntimeCall::XMultiSig(xmultisig::Call::remove_multi_sig_for::<Runtime>(multisig, id));
        self.submit_call(pair, func, acc)
    }

    fn nominate(&self, key: Hash, target: Hash, value: u64, memo: String, acc: u32) -> BoxFuture<Value> {
        let pair = Pair::from_seed(key.into_inner().as_fixed_bytes());
        let target: chainx_runtime::Address = AccountId::from_h256(target.into_inner()).into();
        let memo = memo.into_bytes();
        let func = RuntimeCall::XStaking(xstaking::Call::nominate::<Runtime>(target, value, memo));
        self.submit_call(pair, func, acc)
    }

    fn unnominate(&self, key: Hash, target: Hash, value: u64, memo: String, acc: u32) -> BoxFuture<Value> {
        let pair = Pair::from_seed(key.into_inner().as_fixed_bytes());
        let target: chainx_runtime::Address = AccountId::from_h256(target.into_inner()).into();
        let memo = memo.into_bytes();
        let func = RuntimeCall::XStaking(xstaking::Call::unnominate::<Runtime>(target, value, memo));
        self.submit_call(pair, func, acc)
    }

    fn renominate(&self, key: Hash, from: Hash, to: Hash, value: u64, memo: String, acc: u32) -> BoxFuture<Value> {
        let pair = Pair::from_seed(key.into_inner().as_fixed_bytes());
        let from: chainx_runtime::Address = AccountId::from_h256(from.into_inner()).into();
        let to: chainx_runtime::Address = AccountId::from_h256(to.into_inner()).into();
        let memo = memo.into_bytes();
        let func = RuntimeCall::XStaking(xstaking::Call::renominate::<Runtime>(from, to, value, memo));
        self.submit_call(pair, func, acc)
    }

    fn unfreeze(&self, key: Hash, target: Hash, index: u32, acc: u32) -> BoxFuture<Value> {
        let pair = Pair::from_seed(key.into_inner().as_fixed_bytes());
        let target: chainx_runtime::Address = AccountId::from_h256(target.into_inner()).into();
        let func = RuntimeCall::XStaking(xstaking::Call::unfreeze::<Runtime>(target, index));
        self.submit_call(pair, func, acc)
    }

    fn vote_claim(&self, key: Hash, target: Hash, acc: u32) -> BoxFuture<Value> {
        let pair = Pair::from_seed(key.into_inner().as_fixed_bytes());
        let target: chainx_runtime::Address = AccountId::from_h256(target.into_inner()).into();
        let func = RuntimeCall::XStaking(xstaking::Call::claim::<Runtime>(target));
        self.submit_call(pair, func, acc)
    }

    fn deposit_claim(&self, key: Hash, token: Token, acc: u32) -> BoxFuture<Value> {
        let pair = Pair::from_seed(key.into_inner().as_fixed_bytes());
        let token = token.name();
        let func = RuntimeCall::XTokens(xtokens::Call::claim::<Runtime>(token));
        self.submit_call(pair, func, acc)
    }
}

#[allow(unused)]
#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
enum LocalKey<'a> {
    Seed([u8; 32]),
    RawSeed(&'a str),
    Ed25519(ed25519::Pair),
}

struct RawSeed<'a>(&'a str);

impl<'a> RawSeed<'a> {
    fn new(seed: &'a str) -> Self {
        RawSeed(seed)
    }

    // Unsafe, for test only
    fn pair(&self) -> Pair {
        let seed = &self.0;
        let mut s: [u8; 32] = [b' '; 32];
        let len = std::cmp::min(32, seed.len());
        s[..len].copy_from_slice(&seed.as_bytes()[..len]);
        Pair::from_seed(&s)
    }
}

fn gen_extrinsic(
    key: LocalKey,
    index: Index,
    function: RuntimeCall,
    era: Era,
    hash: Hash,
    acc: Acceleration,
) -> String {
    let pair = match key {
        LocalKey::Seed(seed) => Pair::from_seed(&seed),
        LocalKey::RawSeed(raw_seed) => RawSeed::new(raw_seed).pair(),
        LocalKey::Ed25519(pair) => pair,
    };
    let sender = pair.public();

    let signed: chainx_runtime::Address = sender.into();
    let index = Compact::<Index>::from(index);
    let acc = Compact::<Acceleration>::from(acc);
    let payload = (index, function.clone(), era, hash.into_inner(), acc);
    let signature = payload.using_encoded(|data| {
        if data.len() > 256 {
            pair.sign(&blake2_256(data))
        } else {
            pair.sign(data)
        }
    });

    // signature，func | signature：(sender, signature, index, era, acceleration)
    let unchecked_extrinsic = chainx_runtime::UncheckedExtrinsic {
        signature: Some((signed, signature, index, era, acc)),
        function,
    };
    format!("0x{:}", HexDisplay::from(&unchecked_extrinsic.encode()))
}
