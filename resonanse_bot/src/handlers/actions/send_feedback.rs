use std::env;

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

pub async fn handle_send_feedback(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    info!("got feedback {:?}", msg);

    let manager_bot = MANAGER_BOT.get().ok_or("Cannot get manager bot")?;

    bot.send_message(msg.chat.id, "Спасибо за оставленный фидбек!")
        .await?;

    if let Ok(tg_feedback_chan) = env::var(FEEDBACK_CHANNEL_ID) {
        if let Ok(tg_feedback_chan) = tg_feedback_chan.parse::<i64>() {
            let tg_feedback_chan = ChatId(tg_feedback_chan);
            if let Some(feedback_photo) = msg.photo().and_then(|p| p.last()) {
                let local_img_path = get_feedback_images_path().join(&feedback_photo.file.id);
                download_file_by_id(&bot, &feedback_photo.file.id, &local_img_path).await?;

                let mut feedback_msg =
                    manager_bot.send_photo(tg_feedback_chan, InputFile::file(local_img_path));

                feedback_msg.caption = Some(format!(
                    "Feedback from {}:\n\n{}",
                    repr_user_as_str(msg.from()),
                    markdown::escape(msg.caption().unwrap_or("")),
                ));
                feedback_msg.await?;
                return Ok(());
            }

            if let Some(feedback_text) = msg.text() {
                let mut feedback_msg = manager_bot.send_message(
                    tg_feedback_chan,
                    format!(
                        "Feedback from {}:\n\n{}",
                        repr_user_as_str(msg.from()),
                        markdown::escape(feedback_text),
                    ),
                );

                feedback_msg.await?;
                return Ok(());
            }
        }
    }

    Ok(())
}
