use crate::ReasonCode;
use futures::io::Error as AsyncIOError;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as StdResult,
};

/// Standard Result type for Sage MQTT
pub type Result<T> = StdResult<T, Error>;

/// The error type for Sage MQTT operations
#[derive(Debug)]
pub enum Error {
    /// Standard Rust IO Error
    Io(AsyncIOError),

    /// Error described using a MQTT Reason code
    Reason(ReasonCode),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Error::Reason(rc) => write!(f, "{:?}", rc),
            Error::Io(ref e) => e.fmt(f),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Error::Io(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<AsyncIOError> for Error {
    fn from(err: AsyncIOError) -> Self {
        Error::Io(err)
    }
}

impl From<ReasonCode> for Error {
    fn from(rc: ReasonCode) -> Self {
        Error::Reason(rc)
    }
}
