use log::{debug, info};
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::{InputFile, ParseMode};
use teloxide::utils::markdown;

use resonanse_common::file_storage::get_feedback_images_path;
use resonanse_common::RecServiceClient;

use crate::{ACCOUNTS_REPOSITORY, EVENTS_REPOSITORY, MANAGER_BOT, REC_SERVICE_CLIENT};
use crate::config::{FEEDBACK_CHANNEL_ID, RABBITMQ_HOST};
use crate::data_translators::fill_base_account_from_teloxide_user;
use crate::handlers::{HandlerResult, MyDialogue};
use crate::handlers::utils::download_file_by_id;
use crate::states::BaseState;
use crate::utils::repr_user_as_str;

pub async fn handle_set_city(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    // 1. get available cities from postgres
    let events_repo = EVENTS_REPOSITORY
        .get()
        .ok_or("Cannot get events repository")?;
    let accounts_repo = ACCOUNTS_REPOSITORY
        .get()
        .ok_or("Cannot get accounts repository")?;

    let available_cities = events_repo.get_unique_cities().await?;

    // 2. check is provided city ok
    let reject_user_input_msg = "Выбери город из списка";
    let chosen_city = match msg.text() {
        None => {
            let mut message = bot.send_message(
                msg.chat.id,
                reject_user_input_msg,
            );
            message.await?;
            return Ok(());
        }
        Some(msg_text) => {
            if available_cities.contains(&msg_text.to_string()) {
                msg_text
            } else {
                let mut message = bot.send_message(
                    msg.chat.id,
                    reject_user_input_msg,
                );
                message.await?;
                return Ok(());
            }
        }
    };

    // 3. if ok, save to db and response OK
    let user_id = accounts_repo.get_account_id_by_tg_user_id(
        msg.chat.id.0,
    ).await?;
    accounts_repo.set_user_city(user_id, chosen_city.to_string()).await?;
    dialogue.update(BaseState::Idle).await?;

    let mut message = bot.send_message(
        msg.chat.id,
        format!(
            "Выбран город: {}",
            chosen_city,
        ),
    );
    message.reply_markup = Some(teloxide::types::ReplyMarkup::KeyboardRemove(
        teloxide::types::KeyboardRemove::new()
    ));
    message.await?;

    Ok(())
}

pub async fn handle_set_description(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    // 0. check user exists
    let user = msg.from();
    let accounts_repo = ACCOUNTS_REPOSITORY
        .get()
        .ok_or("Cannot get accounts repository")?;
    if let Some(user) = user {
        let new_user_account = fill_base_account_from_teloxide_user(user);
        accounts_repo.create_user_by_tg_user_id(new_user_account).await?;
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

    let user_id = accounts_repo.get_account_id_by_tg_user_id(
        msg.chat.id.0,
    ).await?;
    let status = rec_service_client
        .rpc_set_user_description(user_id, msg_text)
        .await?;

    // 2. wait for response and send OK
    dialogue.update(BaseState::Idle).await?;
    debug!("rpc_set_user_description status {:?}", status);

    bot.send_message(msg.chat.id, "Ваш рекомендательный запрос перенастроен")
        .await?;

    Ok(())
}
