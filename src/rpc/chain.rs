use serde_json::Value;

use crate::transport::{BoxFuture, ChainXTransport};
use crate::types::Hash;

impl_rpc! {
    pub trait ChainRpc for ChainXTransport<T> {
        "chain_getHeader" => fn header(&self, hash: Option<Hash>) -> BoxFuture<Value>;
        "chain_getFinalizedHead" => fn finalized_head(&self) -> BoxFuture<Value>;
        "chain_getBlockHash" => fn block_hash(&self, number: Option<u64>) -> BoxFuture<Value>;
        "chain_getBlock" => fn block_by_hash(&self, hash: Option<Hash>) -> BoxFuture<Value>;
    }
}
