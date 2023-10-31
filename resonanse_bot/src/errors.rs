use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BotHandlerError {
    UnknownHandler,
}

impl Display for BotHandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BotHandlerError::UnknownHandler => write!(f, "UnknownHandler"),
        }
    }
}

impl Error for BotHandlerError {}
