use std::fmt;
use std::error::Error;
use serde::ser::{Serialize, Serializer};
use diesel;
use r2d2;

#[derive(Debug)]
pub enum ModelError {
    DieselError(diesel::result::Error),
    DieselConnectionError(diesel::result::ConnectionError),
    ConnectionPoolError(r2d2::GetTimeout),
    UserExists
}

impl<'v> Serialize for ModelError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S:Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<diesel::result::Error> for ModelError {
    fn from(err: diesel::result::Error) -> ModelError {
        ModelError::DieselError(err)
    }
}

impl From<diesel::result::ConnectionError> for ModelError {
    fn from(err: diesel::result::ConnectionError) -> ModelError {
        ModelError::DieselConnectionError(err)
    }
}

impl From<r2d2::GetTimeout> for ModelError {
    fn from(err: r2d2::GetTimeout) -> ModelError {
        ModelError::ConnectionPoolError(err)
    }
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModelError::DieselError(ref err) => err.fmt(f),
            ModelError::DieselConnectionError(ref err) => err.fmt(f),
            ModelError::ConnectionPoolError(ref err) => err.fmt(f),
            ref ownerror => write!(f, "{}", ownerror.description())
        }
    }
}

impl Error for ModelError {
    fn description(&self) -> &str {
        match *self {
            ModelError::DieselError(ref err) => err.description(),
            ModelError::DieselConnectionError(ref err) => err.description(),
            ModelError::ConnectionPoolError(ref err) => err.description(),
            ModelError::UserExists => "user already exists"
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            ModelError::DieselError(ref err) => Some(err),
            ModelError::DieselConnectionError(ref err) => Some(err),
            ModelError::ConnectionPoolError(ref err) => Some(err),
            ModelError::UserExists => None
        }
    }
}

pub type ModelResult<T> = Result<T, ModelError>;
