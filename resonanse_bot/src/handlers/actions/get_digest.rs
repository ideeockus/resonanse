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

pub async fn handle_get_digest_command(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    // 1. send rpc request

    // 2. prepare text view

    // 3. send to user

    Ok(())
}
