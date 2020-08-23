// pub mod storage;

use serde_json::Value;

use crate::error::Result;
use crate::transport::ChainXTransport;
use crate::types::Hash;

impl_rpc! {
    pub async trait StateRpc for ChainXTransport<T> {
        "state_getStorage" => fn storage(&self, key: String, hash: Option<Hash>) -> Result<Value>;
        "state_getRuntimeVersion" => fn runtime_version(&self, hash: Option<Hash>) -> Result<Value>;
    }
}
