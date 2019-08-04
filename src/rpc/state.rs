use serde_json::Value;
use web3::BatchTransport;

use crate::transport::{BoxFuture, ChainXTransport};
use crate::types::{/*DecodeWrapper, EncodeWrapper, */Hash};
use crate::util;

pub trait StateRpc {
    /*
    fn storage(
        &self,
        key: EncodeWrapper,
        hash: Option<Hash>,
    ) -> BoxFuture<Option<DecodeWrapper>>;
    */

    fn runtime_version(&self, hash: Option<Hash>) -> BoxFuture<Value>;
}

impl<T: BatchTransport + 'static> StateRpc for ChainXTransport<T> {
    /*
    fn storage(
        &self,
        key: EncodeWrapper,
        hash: Option<Hash>,
    ) -> BoxFuture<Option<DecodeWrapper>> {
        Box::new(
            self.execute(
                "state_getStorage",
                vec![util::serialize(key), util::serialize(hash)],
            )
            .and_then(util::deserialize),
        )
    }
    */

    fn runtime_version(&self, hash: Option<Hash>) -> BoxFuture<Value> {
        self.execute("state_getRuntimeVersion", vec![util::serialize(hash)])
    }
}
