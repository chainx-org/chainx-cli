use serde_json::Value;

use crate::error::Result;
use crate::transport::ChainXTransport;
use crate::types::Hash;

impl_rpc! {
    pub async trait ChainRpc for ChainXTransport<T> {
        "chain_getHeader" => fn header(&self, hash: Option<Hash>) -> Result<Value>;
        "chain_getFinalizedHead" => fn finalized_head(&self) -> Result<Value>;
        "chain_getBlockHash" => fn block_hash(&self, number: Option<u64>) -> Result<Value>;
        "chain_getBlock" => fn block_by_hash(&self, hash: Option<Hash>) -> Result<Value>;
    }
}
