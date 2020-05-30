use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Format error: {0}")]
    Fmt(#[from] std::fmt::Error),
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Rpc error: {0}")]
    Rpc(#[from] web3::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
