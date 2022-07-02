use std::{error, fmt, io};

pub type TDLibError = crate::types::Error;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    SerdeJson(serde_json::Error),
    TDLibError(TDLibError),
    Internal(&'static str),
    BadRequest(&'static str),
}

#[deprecated]
pub type RTDError = Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[deprecated]
pub type RTDResult<T, E = Error> = Result<T, E>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => {
                write!(f, "{}", err)
            }
            Error::SerdeJson(err) => {
                write!(f, "{}", err)
            }
            Error::TDLibError(err) => {
                write!(f, "{:?}", err)
            }
            Error::Internal(err) => {
                write!(f, "{}", err)
            }
            Error::BadRequest(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Error::Io(ref err) => Some(err),
            Error::SerdeJson(ref err) => Some(err),
            Error::Internal(_) => None,
            Error::TDLibError(_) => None,
            Error::BadRequest(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SerdeJson(err)
    }
}

const CLOSED_CHANNEL_ERROR: Error = Error::Internal("channel closed");
const SEND_TO_CHANNEL_TIMEOUT: Error = Error::Internal("timeout for mpsc occurred");

#[cfg(feature = "client")]
impl<T> From<tokio::sync::mpsc::error::SendTimeoutError<T>> for Error {
    fn from(err: tokio::sync::mpsc::error::SendTimeoutError<T>) -> Self {
        match err {
            tokio::sync::mpsc::error::SendTimeoutError::Timeout(_) => SEND_TO_CHANNEL_TIMEOUT,
            tokio::sync::mpsc::error::SendTimeoutError::Closed(_) => CLOSED_CHANNEL_ERROR,
        }
    }
}
