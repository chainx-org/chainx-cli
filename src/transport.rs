use serde_json::Value;
use web3::transports::{Http, WebSocket};
use web3::BatchTransport;

use crate::error::Result;

pub type ChainXWs = ChainXTransport<WebSocket>;
pub async fn ws_connect(url: &str) -> Result<ChainXWs> {
    let transport = WebSocket::new(url).await?;
    Ok(ChainXWs::new(transport))
}

pub type ChainXHttp = ChainXTransport<Http>;
pub fn http_connect(url: &str) -> Result<ChainXHttp> {
    let transport = Http::new(url)?;
    Ok(ChainXHttp::new(transport))
}

/// ChainX RPC Transport
#[derive(Clone)]
pub struct ChainXTransport<T: BatchTransport> {
    transport: T,
}

impl<T: BatchTransport> ChainXTransport<T> {
    pub fn new(transport: T) -> Self {
        ChainXTransport { transport }
    }

    pub async fn execute(&self, method: &str, params: Vec<Value>) -> Result<Value> {
        Ok(self.transport.execute(method, params).await?)
    }
}
