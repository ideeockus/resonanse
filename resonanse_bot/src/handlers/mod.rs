use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use crate::State;

mod commands;
mod middlewares;


type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;