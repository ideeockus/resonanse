use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

pub use actions::*;
pub use commands::*;
pub use common::*;
pub use middlewares::*;
pub use utils::*;

use crate::states::BaseState;

mod actions;
mod base;
mod commands;
mod common;
mod middlewares;
mod utils;

type MyDialogue = Dialogue<BaseState, InMemStorage<BaseState>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
