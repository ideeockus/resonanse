use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[allow(clippy::enum_variant_names)]
pub enum RpcError {
    SqlxError,
    AmqprsError,
    SerdeJsonError,
}

impl From<sqlx::error::Error> for RpcError {
    fn from(_value: sqlx::error::Error) -> Self {
        Self::SqlxError
    }
}

impl From<amqprs::error::Error> for RpcError {
    fn from(_value: amqprs::error::Error) -> Self {
        Self::AmqprsError
    }
}

impl From<serde_json::Error> for RpcError {
    fn from(_value: serde_json::Error) -> Self {
        Self::SerdeJsonError
    }
}

impl Debug for RpcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for RpcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcError::SqlxError => write!(f, "SqlxError"),
            RpcError::AmqprsError => write!(f, "AmqprsError"),
            RpcError::SerdeJsonError => write!(f, "SerdeJsonError"),
        }
    }
}

impl Error for RpcError {}
