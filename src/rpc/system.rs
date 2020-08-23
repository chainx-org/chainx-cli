use serde_json::Value;

use crate::error::Result;
use crate::transport::ChainXTransport;

impl_rpc! {
    pub async trait SystemRpc for ChainXTransport<T> {
        "system_name" => fn system_name(&self) -> Result<Value>;
        "system_version" => fn system_version(&self) -> Result<Value>;
        "system_chain" => fn system_chain(&self) -> Result<Value>;
        "system_properties" => fn system_properties(&self) -> Result<Value>;
        "system_health" => fn system_health(&self) -> Result<Value>;
        "system_peers" => fn system_peers(&self) -> Result<Value>;
        "system_networkState" => fn system_network_state(&self) -> Result<Value>;
    }
}
