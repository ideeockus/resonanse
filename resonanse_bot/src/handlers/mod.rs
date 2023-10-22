use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;
use crate::states::BaseState;
pub use common::*;
pub use commands::*;
pub use base::*;
pub use actions::*;

mod commands;
mod middlewares;
mod common;
mod base;
mod actions;
mod utils;


type MyDialogue = Dialogue<BaseState, InMemStorage<BaseState>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

fn log_request<S>(log_text: S, msg: &Message)
    where S: Into<&str>
{
    log::debug!("{}", log_text.into());
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