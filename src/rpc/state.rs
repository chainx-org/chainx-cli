use serde::{Deserialize, Serialize};
use serde_json::Value;
use web3::futures::Future;
use web3::BatchTransport;

use chainx_primitives::Hash;

use crate::error::Error;
use crate::transport::ChainXTransport;
use crate::util;

impl<T: BatchTransport> ChainXTransport<T> {
    pub fn get_storage(
        &self,
        key: EncodeWrapper,
        hash: Option<Hash>,
    ) -> impl Future<Item = Option<DecodeWrapper>, Error = Error> {
        self.execute(
            "state_getStorage",
            vec![util::serialize(key), util::serialize(hash)],
        )
        .and_then(util::deserialize)
    }

    pub fn get_runtime_version(
        &self,
        hash: Option<Hash>,
    ) -> impl Future<Item = Value, Error = Error> {
        self.execute("state_getRuntimeVersion", vec![util::serialize(hash)])
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EncodeWrapper(substrate_primitives::storage::StorageKey);

#[derive(Clone, Serialize, Deserialize)]
pub struct DecodeWrapper(substrate_primitives::storage::StorageData);
