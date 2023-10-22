use crate::handlers::*;
use log::debug;
use teloxide::Bot;
use crate::states::BaseState;

const HELLO_MSG: &str = r#"
Привет!
"#;

pub async fn handle_start_state(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log_request("got contact (start state) message", &msg);

    dialogue
        .update(BaseState::Idle)
        .await?;

    let message = bot.send_message(
        msg.chat.id,
        HELLO_MSG,
    );
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
    log_request("got message, but state invalid", &msg);

    bot.send_message(
        msg.chat.id,
        "If you got stacked, please read User Guide. Just press /help",
    )
    .await?;

    Ok(())
}
