use serde_json::Value;

use crate::error::Result;
use crate::transport::ChainXTransport;

impl_rpc! {
    pub async trait AuthorRpc for ChainXTransport<T> {
        "author_submitExtrinsic" => fn submit_extrinsic(&self, extrinsic: &str) -> Result<Value>;
    }
}
