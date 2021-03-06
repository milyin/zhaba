use std::fmt;
use std::error::Error;
use serde_json;
use diesel;
use r2d2;

#[derive(Debug)]
pub enum ModelError {
    DieselError(diesel::result::Error),
    DieselConnectionError(diesel::result::ConnectionError),
    ConnectionPoolError(r2d2::GetTimeout),
    SerdeJsonError(serde_json::Error),
    UserExists,
    UserNotFound,
    PasswordWrong,
    AuthTokenExpired,
    AuthTokenInvalid,
    AuthTokenNotFound,
    PostNotFound,
    AccessDenied,
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

impl From<serde_json::Error> for ModelError {
    fn from(err: serde_json::Error) -> ModelError {
        ModelError::SerdeJsonError(err)
    }
}
impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModelError::DieselError(ref err) => err.fmt(f),
            ModelError::DieselConnectionError(ref err) => err.fmt(f),
            ModelError::ConnectionPoolError(ref err) => err.fmt(f),
            ModelError::SerdeJsonError(ref err) => err.fmt(f),
            ref ownerror => write!(f, "{}", ownerror.description()),
        }
    }
}

impl Error for ModelError {
    fn description(&self) -> &str {
        match *self {
            ModelError::DieselError(ref err) => err.description(),
            ModelError::DieselConnectionError(ref err) => err.description(),
            ModelError::ConnectionPoolError(ref err) => err.description(),
            ModelError::SerdeJsonError(ref err) => err.description(),
            ModelError::UserExists => "user already exists",
            ModelError::UserNotFound => "user not found",
            ModelError::PasswordWrong => "wrong password",
            ModelError::AuthTokenExpired => "authentication token expired",
            ModelError::AuthTokenInvalid => "authentication token invalid",
            ModelError::AuthTokenNotFound => "authentication token not found",
            ModelError::PostNotFound => "post not found",
            ModelError::AccessDenied => "access denied",
        }
    }
    fn cause(&self) -> Option<&Error> {
        match *self {
            ModelError::DieselError(ref err) => Some(err),
            ModelError::DieselConnectionError(ref err) => Some(err),
            ModelError::ConnectionPoolError(ref err) => Some(err),
            ModelError::SerdeJsonError(ref err) => Some(err),
            _ => None,
        }
    }
}

