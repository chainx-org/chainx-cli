use serde_json::Value;
use web3::futures::Future;
use web3::BatchTransport;

use crate::transport::{BoxFuture, ChainXTransport};
use crate::util;

pub trait SystemRpc {
    fn system_name(&self) -> BoxFuture<String>;
    fn system_version(&self) -> BoxFuture<String>;
    fn system_chain(&self) -> BoxFuture<String>;
    fn system_properties(&self) -> BoxFuture<Value>;
    fn system_health(&self) -> BoxFuture<Value>;
    fn system_peers(&self) -> BoxFuture<Value>;
    fn system_network_state(&self) -> BoxFuture<Value>;
}

impl<T: BatchTransport + 'static> SystemRpc for ChainXTransport<T> {
    fn system_name(&self) -> BoxFuture<String> {
        Box::new(
            self.execute("system_name", vec![])
                .and_then(util::deserialize),
        )
    }

    fn system_version(&self) -> BoxFuture<String> {
        Box::new(
            self.execute("system_version", vec![])
                .and_then(util::deserialize),
        )
    }

    fn system_chain(&self) -> BoxFuture<String> {
        Box::new(
            self.execute("system_chain", vec![])
                .and_then(util::deserialize),
        )
    }

    fn system_properties(&self) -> BoxFuture<Value> {
        self.execute("system_properties", vec![])
    }

    fn system_health(&self) -> BoxFuture<Value> {
        self.execute("system_health", vec![])
    }

    fn system_peers(&self) -> BoxFuture<Value> {
        self.execute("system_peers", vec![])
    }

    fn system_network_state(&self) -> BoxFuture<Value> {
        self.execute("system_networkState", vec![])
    }
}
