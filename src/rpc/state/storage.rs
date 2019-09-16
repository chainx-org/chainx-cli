use serde::{Deserialize, Serialize};
use serde_json::Value;
use web3::futures::{future, Future};

use chainx_primitives::AccountId;
use chainx_runtime::{Call as RuntimeCall, Runtime};
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

    pub fn addr_info(self) -> xmultisig::AddrInfo<AccountId> {
        parity_codec::Decode::decode(&mut (self.0).0.as_slice()).expect("Decode shouldn't be fail")
    }

    pub fn pending_list(self) -> Vec<substrate_primitives::H256> {
        parity_codec::Decode::decode(&mut (self.0).0.as_slice()).expect("Decode shouldn't be fail")
    }

    pub fn pending_state(self) -> xmultisig::PendingState<RuntimeCall> {
        parity_codec::Decode::decode(&mut (self.0).0.as_slice()).expect("Decode shouldn't be fail")
    }
}

pub trait StorageRpc: StateRpc {
    fn account_nonce(&self, who: Hash, hash: Option<Hash>) -> BoxFuture<u64>;
    fn multisig_addr_info(&self, who: Hash, hash: Option<Hash>) -> BoxFuture<Value>;
    fn multisig_pending_list(&self, who: Hash, hash: Option<Hash>) -> BoxFuture<Value>;
    fn multisig_pending_state(
        &self,
        who: Hash,
        pending_hash: Hash,
        hash: Option<Hash>,
    ) -> BoxFuture<Option<xmultisig::PendingState<RuntimeCall>>>;
}

impl<T: web3::BatchTransport + 'static> StorageRpc for ChainXTransport<T> {
    fn account_nonce(&self, who: Hash, hash: Option<Hash>) -> BoxFuture<u64> {
        let who = AccountId::from_h256(who.into_inner());
        let key = <system::AccountNonce<Runtime>>::key_for(who);
        let key = util::blake2_256_and_hex(&key);
        Box::new(
            self.storage(key, hash)
                .and_then(util::deserialize::<Option<DecodeWrapper>>)
                .and_then(|storage| storage.map_or(future::ok(0), |decoder| future::ok(decoder.nonce()))),
        )
    }

    fn multisig_addr_info(&self, who: Hash, hash: Option<Hash>) -> BoxFuture<Value> {
        let who = AccountId::from_h256(who.into_inner());
        let key = <xmultisig::MultiSigAddrInfo<Runtime>>::key_for(who);
        let key = util::blake2_256_and_hex(&key);
        Box::new(
            self.storage(key, hash)
                .and_then(util::deserialize::<Option<DecodeWrapper>>)
                .and_then(|storage| {
                    storage.map_or(future::ok(Value::Null), |decoder| {
                        future::ok(serde_json::json!(decoder.addr_info()))
                    })
                }),
        )
    }

    fn multisig_pending_list(&self, who: Hash, hash: Option<Hash>) -> BoxFuture<Value> {
        let who = AccountId::from_h256(who.into_inner());
        let key = <xmultisig::PendingListFor<Runtime>>::key_for(who.clone());
        let key = util::blake2_256_and_hex(&key);
        Box::new(
            self.storage(key, hash)
                .and_then(util::deserialize::<Option<DecodeWrapper>>)
                .and_then(|storage| {
                    storage.map_or(future::ok(Value::Array(vec![])), |decoder| {
                        future::ok(serde_json::json!(decoder.pending_list()))
                    })
                }),
        )
    }

    fn multisig_pending_state(
        &self,
        who: Hash,
        pending_hash: Hash,
        hash: Option<Hash>,
    ) -> BoxFuture<Option<xmultisig::PendingState<RuntimeCall>>> {
        let who = AccountId::from_h256(who.into_inner());
        let key = <xmultisig::PendingStateFor<Runtime>>::key_for((who, pending_hash.into_inner()));
        let key = util::blake2_256_and_hex(&key);
        Box::new(
            self.storage(key, hash)
                .and_then(util::deserialize::<Option<DecodeWrapper>>)
                .and_then(|storage| {
                    storage.map_or(future::ok(None), |decoder| future::ok(Some(decoder.pending_state())))
                }),
        )
    }
}
