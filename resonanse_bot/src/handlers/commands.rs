use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use crate::handlers::{HandlerResult, log_request};

const BOT_HELP_TEXT_MD: &str = "Помощ";

pub async fn help(bot: Bot, msg: Message) -> HandlerResult {
    log_request("got help command", &msg);

    let mut message = bot.send_message(msg.chat.id, BOT_HELP_TEXT_MD);
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}

pub async fn create_event(bot: Bot, msg: Message) -> HandlerResult {
    log_request("got create_event command", &msg);

    let mut message = bot.send_message(msg.chat.id, BOT_HELP_TEXT_MD);
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}