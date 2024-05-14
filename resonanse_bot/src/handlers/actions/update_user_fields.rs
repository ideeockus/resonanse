use log::info;
use resonanse_common::file_storage::get_feedback_images_path;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use teloxide::utils::markdown;
use teloxide::Bot;

use crate::config::FEEDBACK_CHANNEL_ID;
use crate::handlers::utils::download_file_by_id;
use crate::handlers::{HandlerResult, MyDialogue};
use crate::utils::repr_user_as_str;
use crate::MANAGER_BOT;

pub async fn handle_set_city(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    // 1. get available cities from postgres

    // 2. check is provided city ok

    // 3. if ok, save to db and response OK

    Ok(())
}

pub async fn handle_set_description(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    // 1. send new description to rpc

    // 2. wait for response and send OK

    Ok(())
}
