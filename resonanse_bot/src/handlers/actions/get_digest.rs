use log::{debug, info};
use resonanse_common::file_storage::get_feedback_images_path;
use resonanse_common::RecServiceClient;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use teloxide::utils::markdown;
use teloxide::Bot;

use crate::config::{FEEDBACK_CHANNEL_ID, RABBITMQ_HOST};
use crate::handlers::utils::download_file_by_id;
use crate::handlers::{HandlerResult, MyDialogue};
use crate::utils::repr_user_as_str;
use crate::{MANAGER_BOT, REC_SERVICE_CLIENT};

pub async fn handle_get_digest_command(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    // 1. send rpc request
    let rec_service_client = REC_SERVICE_CLIENT.get()
        .ok_or("Cannot get rpc service client")?;
    let user_id = msg.chat.id.0;

    // todo  get user_id by tg_user_id
    let recommendation = rec_service_client
        .rpc_get_recommendation_by_user(user_id)
        .await?;

    debug!("recommendation {:?}", recommendation);

    // 2. prepare text view

    // 3. send to user

    Ok(())
}
