use serde_json::Value;

use support::StorageMap;

use crate::rpc::state::StateRpc;
use crate::transport::{BoxFuture, ChainXTransport};
use crate::types::Hash;
use crate::util::blake2_256_and_hex;

pub trait StorageRpc: StateRpc {
    fn account_nonce(&self, who: Hash, hash: Option<Hash>) -> BoxFuture<Value>;
    fn multisig_addr_info(&self, who: Hash, hash: Option<Hash>) -> BoxFuture<Value>;
}

impl<T: web3::BatchTransport + 'static> StorageRpc for ChainXTransport<T> {
    fn account_nonce(&self, who: Hash, hash: Option<Hash>) -> BoxFuture<Value> {
        let who = chainx_primitives::AccountId::from_h256(who.into_inner());
        let key = <system::AccountNonce<chainx_runtime::Runtime>>::key_for(who);
        let key = blake2_256_and_hex(&key);
        self.storage(key, hash)
    }

    fn multisig_addr_info(&self, who: Hash, hash: Option<Hash>) -> BoxFuture<Value> {
        let who = chainx_primitives::AccountId::from_h256(who.into_inner());
        let key = <xmultisig::MultiSigAddrInfo<chainx_runtime::Runtime>>::key_for(who);
        let key = blake2_256_and_hex(&key);
        self.storage(key, hash)
    }
}
