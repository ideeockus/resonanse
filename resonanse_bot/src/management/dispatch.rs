use crate::commands::Command;
use crate::handlers::*;
use crate::states::*;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::prelude::*;

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help_command))
    ;

    let message_handler = Update::filter_message()
        .branch(dptree::endpoint(invalid_state));


    dialogue::enter::<Update, InMemStorage<BaseState>, BaseState, _>()
        .branch(message_handler)
        // .branch(callback_query_handler)
}
