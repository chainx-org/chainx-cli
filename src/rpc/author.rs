use serde_json::Value;

use crate::transport::{BoxFuture, ChainXTransport};

impl_rpc! {
    pub trait AuthorRpc for ChainXTransport<T> {
        "author_submitExtrinsic" => fn submit_extrinsic(&self, extrinsic: &str) -> BoxFuture<Value>;
    }
}
