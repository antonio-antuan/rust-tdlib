use std::{error, fmt, io};

pub type TDLibError = crate::types::Error;

#[derive(Debug)]
pub enum RTDError {
    Io(io::Error),
    SerdeJson(serde_json::Error),
    TDLibError(TDLibError),
    Internal(&'static str),
    BadRequest(&'static str),
}

pub type RTDResult<T, E = RTDError> = Result<T, E>;

impl fmt::Display for RTDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RTDError::Io(err) => {
                write!(f, "{}", err)
            }
            RTDError::SerdeJson(err) => {
                write!(f, "{}", err)
            }
            RTDError::TDLibError(err) => {
                write!(f, "{:?}", err)
            }
            RTDError::Internal(err) => {
                write!(f, "{}", err)
            }
            RTDError::BadRequest(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

impl error::Error for RTDError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            RTDError::Io(ref err) => Some(err),
            RTDError::SerdeJson(ref err) => Some(err),
            RTDError::Internal(_) => None,
            RTDError::TDLibError(_) => None,
            RTDError::BadRequest(_) => None,
        }
    }
}

impl From<io::Error> for RTDError {
    fn from(err: io::Error) -> RTDError {
        RTDError::Io(err)
    }
}

impl From<serde_json::Error> for RTDError {
    fn from(err: serde_json::Error) -> RTDError {
        RTDError::SerdeJson(err)
    }
}

const CLOSED_CHANNEL_ERROR: RTDError = RTDError::Internal("channel closed");
const SEND_TO_CHANNEL_TIMEOUT: RTDError = RTDError::Internal("timeout for mpsc occurred");

#[cfg(feature = "client")]
impl<T> From<tokio::sync::mpsc::error::SendTimeoutError<T>> for RTDError {
    fn from(err: tokio::sync::mpsc::error::SendTimeoutError<T>) -> Self {
        match err {
            tokio::sync::mpsc::error::SendTimeoutError::Timeout(_) => SEND_TO_CHANNEL_TIMEOUT,
            tokio::sync::mpsc::error::SendTimeoutError::Closed(_) => CLOSED_CHANNEL_ERROR,
        }
    }
}
