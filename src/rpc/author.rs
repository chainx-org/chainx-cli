use web3::futures::Future;
use web3::BatchTransport;

use chainx_primitives::Hash;

use crate::error::Error;
use crate::transport::ChainXTransport;
use crate::util;

impl<T: BatchTransport> ChainXTransport<T> {
    pub fn submit_extrinsic(&self, extrinsic: &str) -> impl Future<Item = Hash, Error = Error> {
        self.execute("author_submitExtrinsic", vec![util::serialize(extrinsic)])
            .and_then(util::deserialize)
    }
}
