use crate::commands::Command;
use crate::handlers::*;
use crate::State;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::prelude::*;

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        // .branch(case![Command::Help].endpoint(help))
        // .branch(case![Command::Start].endpoint(start));
        .branch(case![Command::Help].endpoint(handle_start_state));

    let message_handler = Update::filter_message()
        // todo: add logging middleware
        .branch(command_handler)
        .branch(case![State::Start].endpoint(handle_start_state))
        .branch(case![State::WaitProcessPicture { settings }].endpoint(handle_base_action))
        .branch(case![State::WaitPalettePicture { settings }].endpoint(handle_palette_image))
        .branch(case![State::ViewSettings { settings }].endpoint(view_settings))
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query()
        .branch(case![State::WaitProcessPicture { settings }].endpoint(handle_process_mode))
        .branch(dptree::endpoint(invalid_state_callback));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
