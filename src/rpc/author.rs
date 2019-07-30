use web3::futures::Future;
use web3::helpers;
use web3::BatchTransport;

use chainx_primitives::Hash;

use crate::error::Error;
use crate::transport::ChainXTransport;
use crate::util;

impl<T: BatchTransport> ChainXTransport<T> {
    pub fn author_submit_extrinsic(
        &self,
        extrinsic: &str,
    ) -> impl Future<Item = Hash, Error = Error> {
        self.execute(
            "author_submitExtrinsic",
            vec![helpers::serialize(&extrinsic)],
        )
        .and_then(util::deserialize)
    }
}
