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
use crate::{ACCOUNTS_REPOSITORY, MANAGER_BOT, REC_SERVICE_CLIENT};
use crate::data_translators::fill_base_account_from_teloxide_user;

pub async fn handle_set_city(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    // 1. get available cities from postgres

    // 2. check is provided city ok

    // 3. if ok, save to db and response OK

    Ok(())
}

pub async fn handle_set_description(
    bot: Bot,
    _dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    // 0. check user exists
    let user = msg.from();
    if let Some(user) = user {
        let new_user_account = fill_base_account_from_teloxide_user(user);
        ACCOUNTS_REPOSITORY
            .get()
            .ok_or("Cannot get accounts repository")?
            .create_user_by_tg_user_id(new_user_account)
            .await?;
    }

    // 1. send new description to rpc
    let rec_service_client = REC_SERVICE_CLIENT
        .get()
        .ok_or("Cannot get rpc service client")?;

    let msg_text = match msg.text() {
        None => {
            bot.send_message(msg.chat.id, "Нужно прислать текст.")
                .await?;
            return Ok(());
        }
        Some(v) => v,
    };

    let user_id = msg.chat.id.0;
    let status = rec_service_client
        .rpc_set_user_description(user_id, msg_text)
        .await?;

    // 2. wait for response and send OK
    debug!("rpc_set_user_description status {:?}", status);

    bot.send_message(msg.chat.id, "Ваш рекомендательныйй запрос перенастроен")
        .await?;

    Ok(())
}
