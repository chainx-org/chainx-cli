use serde_json::Value;

use crate::transport::{BoxFuture, ChainXTransport};
use crate::types::Hash;

impl_rpc! {
    pub trait StateRpc for ChainXTransport<T> {
        // need to deserialize the result
        //"state_getStorage" => fn storage(&self, key: EncodeWrapper, hash: Option<Hash>) -> BoxFuture<Option<DecodeWrapper>>;
        "state_getRuntimeVersion" => fn runtime_version(&self, hash: Option<Hash>) -> BoxFuture<Value>;
    }
}
