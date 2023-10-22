use crate::commands::Command;
use crate::handlers::*;
use crate::states::*;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::prelude::*;
// use teloxide::types::Message;

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        // .branch(case![Command::Help].endpoint(help))
        // .branch(case![Command::Start].endpoint(start));
        .branch(case![Command::Help].endpoint(handle_start_state));

    let message_handler = Update::filter_message()
        // todo: add logging middleware
        .branch(command_handler)
        .branch(case![BaseState::Start].endpoint(handle_start_state))
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query()
        .branch(case![BaseState::GetEventList { page_size, list_page }].endpoint(handle_page_callback))
        .branch(dptree::endpoint(invalid_state_callback));

    dialogue::enter::<Update, InMemStorage<BaseState>, BaseState, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
