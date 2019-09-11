#[cfg(feature = "internal")]
pub mod storage;

use serde_json::Value;

use crate::transport::{BoxFuture, ChainXTransport};
use crate::types::Hash;

impl_rpc! {
    pub trait StateRpc for ChainXTransport<T> {
        "state_getStorage" => fn storage(&self, key: String, hash: Option<Hash>) -> BoxFuture<Value>;
        "state_getRuntimeVersion" => fn runtime_version(&self, hash: Option<Hash>) -> BoxFuture<Value>;
    }
}
