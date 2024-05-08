use log::debug;
use teloxide::types::ParseMode;
use teloxide::Bot;

use crate::handlers::*;
use crate::keyboards;
use crate::states::BaseState;
use crate::utils::repr_user_as_str;

pub async fn handle_start_state(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    // log_request("got contact (start state) message", &msg);

    dialogue.update(BaseState::Idle).await?;

    let mut message = bot.send_message(msg.chat.id, t!("hello_msg"));
    message.parse_mode = Some(ParseMode::MarkdownV2);
    // message.reply_markup = Some(base_keyboard());
    message.await?;

    Ok(())
}

pub async fn invalid_state_callback(bot: Bot, q: CallbackQuery) -> HandlerResult {
    debug!("got invalid callback");

    if let Some(keyboards::INLINE_LIKE_EVENT_BTN | keyboards::INLINE_DISLIKE_EVENT_BTN) =
        q.data.as_deref()
    {
        // handle_like()
        debug!("got like or dislike for event");
        bot.answer_callback_query(q.id).await?;
        return Ok(());
    }

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
