use crate::config::MANAGER_TG_IDS;
use crate::management::actions::*;
use crate::management::commands::ManagementCommand;
use crate::management::common::HandlerResult;
use crate::management::BaseManagementState;
use log::debug;
use std::env;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::prelude::*;

pub fn manager_schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<ManagementCommand, _>()
        .filter(check_is_manager)
        .branch(case![ManagementCommand::DeleteEvent].endpoint(delete_event_command))
        .branch(case![ManagementCommand::GetStatistics].endpoint(get_stats_command))
        .branch(case![ManagementCommand::SearchEventByName(name)].endpoint(search_event_command));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dptree::endpoint(unhandled_message));

    dialogue::enter::<Update, InMemStorage<BaseManagementState>, BaseManagementState, _>()
        .branch(message_handler)
}

fn get_managers_ids() -> Vec<i64> {
    let managers_ids_str = env::var(MANAGER_TG_IDS).unwrap_or("".to_string());
    debug!("managers_ids_str: {:?}", managers_ids_str);
    let managers_ids = managers_ids_str
        .split(',')
        .filter_map(|mng_id_str| mng_id_str.parse::<i64>().ok())
        .collect();

    debug!("managers_ids: {:?}", managers_ids);
    managers_ids
}

pub fn check_is_manager(msg: Message) -> bool {
    // CHECK FOR MANAGER RIGHTS
    if get_managers_ids().contains(&msg.chat.id.0) {
        debug!("User with id {:?} passed as manager", msg.chat.id);
        true
    } else {
        false
    }
}

pub async fn unhandled_message(bot: Bot, msg: Message) -> HandlerResult {
    debug!("got message {:?}", &msg);

    bot.send_message(msg.chat.id, "No handlers").await?;

    Ok(())
}
