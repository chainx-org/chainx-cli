use serde_json::Value;
use web3::futures::Future;
use web3::BatchTransport;

use chainx_primitives::Hash;

use crate::error::Error;
use crate::transport::ChainXTransport;
use crate::util;

impl<T: BatchTransport> ChainXTransport<T> {
    pub fn header(&self, hash: Option<Hash>) -> impl Future<Item = Value, Error = Error> {
        self.execute("chain_getHeader", vec![util::serialize(hash)])
    }

    pub fn block(&self, hash: Option<Hash>) -> impl Future<Item = Value, Error = Error> {
        self.execute("chain_getHeader", vec![util::serialize(hash)])
    }

    pub fn block_hash(&self, number: Option<u64>) -> impl Future<Item = Hash, Error = Error> {
        self.execute("chain_getHeader", vec![util::serialize(number)])
            .and_then(util::deserialize)
    }

    pub fn finalized_head(&self) -> impl Future<Item = Hash, Error = Error> {
        self.execute("chain_getFinalizedHead", vec![])
            .and_then(util::deserialize)
    }
}
