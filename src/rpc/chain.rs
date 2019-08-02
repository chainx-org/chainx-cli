use serde_json::Value;
use web3::futures::Future;
use web3::BatchTransport;

use chainx_primitives::Hash;

use crate::transport::{BoxFuture, ChainXTransport};
use crate::util;

pub trait ChainRpc {
    fn header(&self, hash: Option<Hash>) -> BoxFuture<Value>;

    fn finalized_head(&self) -> BoxFuture<Hash>;

    fn block_hash(&self, number: Option<u64>) -> BoxFuture<Hash>;

    fn block(&self, hash: Option<Hash>) -> BoxFuture<Value>;
}

impl<T: BatchTransport + 'static> ChainRpc for ChainXTransport<T> {
    fn header(&self, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute("chain_getHeader", vec![util::serialize(hash)])
    }

    fn finalized_head(&self) -> BoxFuture<Hash> {
        self.execute("chain_getFinalizedHead", vec![])
            .and_then(util::deserialize)
    }

    fn block_hash(&self, number: Option<u64>) -> BoxFuture<Hash> {
        self.execute("chain_getBlockHash", vec![util::serialize(number)])
            .and_then(util::deserialize)
    }

    fn block(&self, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute("chain_getBlock", vec![util::serialize(hash)])
    }
}
