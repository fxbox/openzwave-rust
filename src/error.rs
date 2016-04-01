pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InitError(&'static str),
    APIError(&'static str),
    GetError(GetSetError),
    SetError(GetSetError)
}

#[derive(Debug)]
pub enum GetSetError {
    APIError(&'static str),
    WrongType,
    InvalidString
}

use std::fmt;
use std::error;
impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let str = match *self {
            Error::InitError(ref str) | Error::APIError(ref str) => {
                format!("{}: {}", error::Error::description(self), str)
            },
            Error::GetError(ref specific_error) | Error::SetError(ref specific_error) => {
                format!("{}: {:?}", error::Error::description(self), specific_error)
            }
        };
        write!(formatter, "{}", str)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InitError(_) => "Initialization Error",
            Error::APIError(_) => "OpenZWave C++ library Error",
            Error::GetError(_) => "Error getting a value",
            Error::SetError(_) => "Error setting a value"
        }
    }
}

use std::ffi::NulError;
impl From<NulError> for Error {
    fn from(_: NulError) -> Error {
        Error::SetError(GetSetError::InvalidString)
    }
}
