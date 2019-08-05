use std::fmt;

use failure::{Backtrace, Fail};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(ErrorKind);

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "Format error: {}", _0)]
    Fmt(#[cause] std::fmt::Error),
    #[fail(display = "Io error: {}", _0)]
    Io(#[cause] std::io::Error),
    #[fail(display = "Json error: {}", _0)]
    SerdeJson(#[cause] serde_json::Error),
    #[fail(display = "{}", _0)]
    Rpc(#[cause] web3::Error),
}

impl Fail for Error {
    fn name(&self) -> Option<&str> {
        self.0.name()
    }

    fn cause(&self) -> Option<&dyn Fail> {
        self.0.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.0.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error(kind)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Self {
        Error::from(ErrorKind::Fmt(err))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::from(ErrorKind::Io(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::from(ErrorKind::SerdeJson(err))
    }
}

impl From<web3::Error> for Error {
    fn from(err: web3::Error) -> Self {
        Error::from(ErrorKind::Rpc(err))
    }
}
