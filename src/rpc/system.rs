use serde_json::Value;
use web3::futures::Future;
use web3::BatchTransport;

use crate::error::Error;
use crate::transport::ChainXTransport;
use crate::util;

impl<T: BatchTransport> ChainXTransport<T> {
    pub fn system_name(&self) -> impl Future<Item = String, Error = Error> {
        self.execute("system_name", vec![])
            .and_then(util::deserialize)
    }

    pub fn system_version(&self) -> impl Future<Item = String, Error = Error> {
        self.execute("system_name", vec![])
            .and_then(util::deserialize)
    }

    pub fn system_chain(&self) -> impl Future<Item = String, Error = Error> {
        self.execute("system_name", vec![])
            .and_then(util::deserialize)
    }

    pub fn system_properties(&self) -> impl Future<Item = Value, Error = Error> {
        self.execute("system_properties", vec![])
    }

    pub fn system_health(&self) -> impl Future<Item = Value, Error = Error> {
        self.execute("system_health", vec![])
    }

    pub fn system_peers(&self) -> impl Future<Item = Value, Error = Error> {
        self.execute("system_peers", vec![])
    }

    pub fn system_network_state(&self) -> impl Future<Item = Value, Error = Error> {
        self.execute("system_networkState", vec![])
    }
}
