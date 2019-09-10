use serde_json::Value;

use crate::transport::{BoxFuture, ChainXTransport};

impl_rpc! {
    pub trait SystemRpc for ChainXTransport<T> {
        "system_name" => fn system_name(&self) -> BoxFuture<Value>;
        "system_version" => fn system_version(&self) -> BoxFuture<Value>;
        "system_chain" => fn system_chain(&self) -> BoxFuture<Value>;
        "system_properties" => fn system_properties(&self) -> BoxFuture<Value>;
        "system_health" => fn system_health(&self) -> BoxFuture<Value>;
        "system_peers" => fn system_peers(&self) -> BoxFuture<Value>;
        "system_network_state" => fn system_network_state(&self) -> BoxFuture<Value>;
    }
}
