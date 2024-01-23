use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum BotHandlerError {
    UnknownHandler,
    UserInputRejected,
    UnfilledEvent,
}

impl Display for BotHandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BotHandlerError::UnknownHandler => write!(f, "UnknownHandler"),
            BotHandlerError::UserInputRejected => write!(f, "UserInputRejected"),
            BotHandlerError::UnfilledEvent => write!(f, "UnfilledEvent"),
        }
    }
}

impl Error for BotHandlerError {}
