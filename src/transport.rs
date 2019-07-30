use web3::futures::{future, Future};
use web3::transports::{EventLoopHandle, Http, WebSocket};
use web3::BatchTransport;

use crate::error::{Error, Result};

pub type ChainXWs = ChainXTransport<WebSocket>;

pub fn ws_connect(url: &str) -> Result<(EventLoopHandle, ChainXWs)> {
    let (handle, transport) = WebSocket::new(url)?;
    Ok((handle, ChainXWs::new(transport)))
}

pub type ChainXHttp = ChainXTransport<Http>;

pub fn http_connect(url: &str) -> Result<(EventLoopHandle, ChainXHttp)> {
    let (handle, transport) = Http::new(url)?;
    Ok((handle, ChainXHttp::new(transport)))
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

    pub fn submit_batch(
        &self,
        method: &str,
        batch_params: Vec<Vec<serde_json::Value>>,
    ) -> impl Future<Item = Vec<Result<serde_json::Value>>, Error = Error> {
        let requests = batch_params
            .into_iter()
            .map(|params| (method, params))
            .collect::<Vec<_>>();
        self.submit_batch_opt(requests)
    }

    pub fn submit_batch_opt(
        &self,
        requests: Vec<(&str, Vec<serde_json::Value>)>,
    ) -> impl Future<Item = Vec<Result<serde_json::Value>>, Error = Error> {
        let requests = requests
            .into_iter()
            .map(|(method, params)| self.transport.prepare(method, params))
            .collect::<Vec<_>>();
        self.transport
            .send_batch(requests)
            .map_err(Into::into)
            .and_then(|responses| {
                let responses = responses
                    .into_iter()
                    .map(|response| response.map_err(Into::into))
                    .collect::<Vec<_>>();
                future::ok(responses)
            })
    }

    pub fn execute(
        &self,
        method: &str,
        params: Vec<serde_json::Value>,
    ) -> impl Future<Item = serde_json::Value, Error = Error> {
        self.transport.execute(method, params).map_err(Into::into)
    }
}
