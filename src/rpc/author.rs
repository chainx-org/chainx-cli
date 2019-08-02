use web3::futures::Future;
use web3::BatchTransport;

use chainx_primitives::Hash;

use crate::transport::{BoxFuture, ChainXTransport};
use crate::util;

pub trait AuthorRpc {
    fn submit_extrinsic(&self, extrinsic: &str) -> BoxFuture<Hash>;
}

impl<T: BatchTransport + 'static> AuthorRpc for ChainXTransport<T> {
    fn submit_extrinsic(&self, extrinsic: &str) -> BoxFuture<Hash> {
        self.execute("author_submitExtrinsic", vec![util::serialize(extrinsic)])
            .and_then(util::deserialize)
    }
}
