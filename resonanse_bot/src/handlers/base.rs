use teloxide::Bot;
use teloxide::prelude::*;

use crate::handlers::{HandlerResult, log_request, MyDialogue};

pub async fn handle_page_callback(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log_request("got handle_page_callback message", &msg);

    // dialogue
    //     .update(BaseState::Idle)
    //     .await?;

    let message = bot.send_message(
        msg.chat.id,
        format!("handled"),
    );
    // message.reply_markup = Some(base_keyboard());
    message.await?;

    Ok(())
}