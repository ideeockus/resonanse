use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use crate::State;
pub use common::*;
pub use commands::*;
pub use base::*;

mod commands;
mod middlewares;
mod common;
mod base;


type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;