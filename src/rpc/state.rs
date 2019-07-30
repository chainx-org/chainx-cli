use serde::{Deserialize, Serialize};

use web3::futures::Future;
use web3::BatchTransport;
use web3::helpers;

use chainx_primitives::Hash;

use crate::error::Error;
use crate::transport::ChainXTransport;
use crate::util;

impl<T: BatchTransport> ChainXTransport<T> {
    pub fn state_get_storage(
        &self,
        params: Vec<serde_json::Value>,
    ) -> impl Future<Item = Option<DecodeWrapper>, Error = Error> {
        self.execute("state_getStorage", params)
            .and_then(util::deserialize)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DecodeWrapper(substrate_primitives::storage::StorageData);
