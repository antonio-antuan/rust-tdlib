use std::{error, fmt, io};

#[derive(Debug)]
pub enum RTDError {
    Io(io::Error),
    SerdeJson(serde_json::Error),
    TdlibError(String),
    Internal(&'static str),
    InvalidParameters(&'static str),
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
            RTDError::TdlibError(err) => {
                write!(f, "{}", err)
            }
            RTDError::Internal(err) => {
                write!(f, "{}", err)
            }
            RTDError::InvalidParameters(err) => {
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
            RTDError::TdlibError(_) => None,
            RTDError::InvalidParameters(_) => None,
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
