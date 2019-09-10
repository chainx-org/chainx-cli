use std::fmt;

use failure::Fail;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Format error: {}", _0)]
    Fmt(#[cause] std::fmt::Error),
    #[fail(display = "Io error: {}", _0)]
    Io(#[cause] std::io::Error),
    #[fail(display = "Json error: {}", _0)]
    Json(#[cause] serde_json::Error),
    #[fail(display = "{}", _0)]
    Rpc(#[cause] web3::Error),
}

impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Self {
        Error::Fmt(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl From<web3::Error> for Error {
    fn from(err: web3::Error) -> Self {
        Error::Rpc(err)
    }
}
