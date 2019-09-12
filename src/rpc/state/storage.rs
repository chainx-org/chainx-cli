use serde::{Deserialize, Serialize};
use serde_json::Value;

use chainx_primitives::AccountId;
use chainx_runtime::Runtime;
use support::StorageMap;

use crate::rpc::state::StateRpc;
use crate::transport::{BoxFuture, ChainXTransport};
use crate::types::Hash;
use crate::util;

#[derive(Clone, Serialize, Deserialize)]
pub struct DecodeWrapper(substrate_primitives::storage::StorageData);

impl DecodeWrapper {
    pub fn nonce(self) -> u64 {
        parity_codec::Decode::decode(&mut (self.0).0.as_slice()).expect("Decode shouldn't be fail")
    }
}

pub trait StorageRpc: StateRpc {
    fn account_nonce(&self, who: Hash, hash: Option<Hash>) -> BoxFuture<Value>;
    fn multisig_addr_info(&self, who: Hash, hash: Option<Hash>) -> BoxFuture<Value>;
}

impl<T: web3::BatchTransport + 'static> StorageRpc for ChainXTransport<T> {
    fn account_nonce(&self, who: Hash, hash: Option<Hash>) -> BoxFuture<Value> {
        let who = AccountId::from_h256(who.into_inner());
        let key = <system::AccountNonce<Runtime>>::key_for(who);
        let key = util::blake2_256_and_hex(&key);
        self.storage(key, hash)
    }

    fn multisig_addr_info(&self, who: Hash, hash: Option<Hash>) -> BoxFuture<Value> {
        let who = AccountId::from_h256(who.into_inner());
        let key = <xmultisig::MultiSigAddrInfo<Runtime>>::key_for(who);
        let key = util::blake2_256_and_hex(&key);
        self.storage(key, hash)
    }
}
