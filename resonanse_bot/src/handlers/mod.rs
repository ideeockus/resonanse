use crate::states::BaseState;
pub use actions::*;

pub use commands::*;
pub use common::*;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

mod actions;
mod base;
mod commands;
mod common;
mod middlewares;
mod utils;

type MyDialogue = Dialogue<BaseState, InMemStorage<BaseState>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

fn log_request<S>(log_text: S, msg: &Message)
where
    S: ToString,
{
    log::debug!("{}", log_text.to_string());
    match msg.from() {
        None => {
            log::debug!("message from unknown user");
        }
        Some(user) => {
            log::debug!(
                "message from user {:?} [{}] - {}. special: {}|{}",
                user.mention(),
                user.id,
                user.full_name(),
                user.is_anonymous(),
                user.is_telegram(),
            );
        }
    }
}
