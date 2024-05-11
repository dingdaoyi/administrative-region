

use std::fmt::{Display, Formatter};
use std::io::Error;
/// 平台统一异常
#[derive(Debug)]
pub enum ServerError {
    ///常规异常
    Message(String),
    IoError(String),
    SqlxError(sqlx::Error),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::Message(msg) => {
                write!(f, "通用异常: {}", msg)
            }
            ServerError::IoError(msg) => {
                write!(f, "IO异常: {}", msg)
            }
            ServerError::SqlxError(error) => {
                write!(f, "IO异常: {:?}", error)
            }
        }
    }
}

impl std::error::Error for ServerError {}

impl From<sqlx::Error> for ServerError{
    fn from(value: sqlx::Error) -> Self {
        ServerError::SqlxError(value)
    }
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}





impl From<std::io::Error> for ServerError {
    fn from(value: Error) -> Self {
        ServerError::IoError(value.to_string())
    }
}

