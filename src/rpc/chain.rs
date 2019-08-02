use serde_json::Value;
use web3::BatchTransport;

use crate::transport::{BoxFuture, ChainXTransport};
use crate::types::Hash;
use crate::util;

pub trait ChainRpc {
    fn header(&self, hash: Option<Hash>) -> BoxFuture<Value>;

    fn finalized_head(&self) -> BoxFuture<Value>;

    fn block_hash(&self, number: Option<u64>) -> BoxFuture<Value>;

    fn block_by_hash(&self, hash: Option<Hash>) -> BoxFuture<Value>;
}

impl<T: BatchTransport + 'static> ChainRpc for ChainXTransport<T> {
    fn header(&self, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute("chain_getHeader", vec![util::serialize(hash)])
    }

    fn finalized_head(&self) -> BoxFuture<Value> {
        self.execute("chain_getFinalizedHead", vec![])
    }

    fn block_hash(&self, number: Option<u64>) -> BoxFuture<Value> {
        self.execute("chain_getBlockHash", vec![util::serialize(number)])
    }

    fn block_by_hash(&self, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute("chain_getBlock", vec![util::serialize(hash)])
    }
}
