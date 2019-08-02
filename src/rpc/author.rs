use serde_json::Value;
use web3::BatchTransport;

use crate::transport::{BoxFuture, ChainXTransport};
use crate::util;

pub trait AuthorRpc {
    fn submit_extrinsic(&self, extrinsic: &str) -> BoxFuture<Value>;
}

impl<T: BatchTransport + 'static> AuthorRpc for ChainXTransport<T> {
    fn submit_extrinsic(&self, extrinsic: &str) -> BoxFuture<Value> {
        self.execute("author_submitExtrinsic", vec![util::serialize(extrinsic)])
    }
}
