use crate::handlers::*;
use crate::states::BaseState;
use crate::utils::repr_user_as_str;
use log::debug;
use teloxide::types::ParseMode;
use teloxide::Bot;

// const HELLO_MSG: &str = r#"
// Привет!
// "#;

pub async fn handle_start_state(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    // log_request("got contact (start state) message", &msg);

    dialogue.update(BaseState::Idle).await?;

    let mut message = bot.send_message(msg.chat.id, HELLO_MESSAGE_MD);
    message.parse_mode = Some(ParseMode::MarkdownV2);
    // message.reply_markup = Some(base_keyboard());
    message.await?;

    Ok(())
}

pub async fn invalid_state_callback(bot: Bot, q: CallbackQuery) -> HandlerResult {
    debug!("got invalid callback");
    if let Some(msg) = q.message {
        bot.delete_message(q.from.id, msg.id).await?;
    }
    bot.answer_callback_query(q.id).await?;

    Ok(())
}

pub async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    // log_request("got message, but state invalid", &msg);
    debug!(
        "unhandled message from {}",
        repr_user_as_str(msg.from()),
        // msg
    );

    bot.send_message(
        msg.chat.id,
        "Если ты застрял, можешь вернуться и почитать мини-гайд /start",
    )
    .await?;

    Ok(())
}
