use crate::handlers::*;
use crate::management::actions::*;
use crate::management::commands::ManagementCommand;
use crate::management::BaseManagementState;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::prelude::*;

pub fn manager_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<ManagementCommand, _>()
        .branch(case![ManagementCommand::DeleteEvent].endpoint(delete_event_command))
        .branch(case![ManagementCommand::GetStatistics].endpoint(get_stats_command))
        .branch(case![ManagementCommand::SearchEventByName(name)].endpoint(search_event_command))
        ;

    let message_handler = Update::filter_message().branch(command_handler)
        // .branch(dptree::endpoint(handle_common_message))
        ;

    dialogue::enter::<Update, InMemStorage<BaseManagementState>, BaseManagementState, _>()
        .branch(message_handler)
}
