use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use crate::ACCOUNTS_REPOSITORY;
use crate::data_translators::fill_base_account_from_teloxide_user;
use crate::handlers::{FillingEvent, HandlerResult, log_request, MyDialogue};
use crate::states::{BaseState, CreateEventState};

const BOT_HELP_TEXT_MD: &str = "Помощ";
const CREATE_EVENT_TEXT_MD: &str = "Введите название события";

pub async fn start_command(bot: Bot, msg: Message) -> HandlerResult {
    log_request("got start_command", &msg);

    let mut message = bot.send_message(msg.chat.id, "Начальное сообщение");
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    if let Some(user) = msg.from() {
        let new_user_account = fill_base_account_from_teloxide_user(user);
        ACCOUNTS_REPOSITORY.get()
            .ok_or("Cannot get accounts repository")?
            .create_user_by_tg_user_id(new_user_account)
            .await?;
    }

    Ok(())
}

pub async fn about_command(bot: Bot, msg: Message) -> HandlerResult {
    log_request("got about_command", &msg);

    let mut message = bot.send_message(msg.chat.id, BOT_HELP_TEXT_MD);
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}

pub async fn create_event_command(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log_request("got create_event command", &msg);

    let mut message = bot.send_message(msg.chat.id, CREATE_EVENT_TEXT_MD);
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    let creator_user_id = msg.from().ok_or("Cannot get user from message")?.id;

    dialogue.update(
        BaseState::CreateEvent {
            state: CreateEventState::Name,
            filling_event: FillingEvent::new(),
        }
    ).await?;

    Ok(())
}

pub async fn get_events_command(bot: Bot, msg: Message) -> HandlerResult {
    log_request("got get_events command", &msg);

    let mut message = bot.send_message(msg.chat.id, "This feature unsupported");
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}

pub async fn send_feedback_command(bot: Bot, msg: Message) -> HandlerResult {
    log_request("got send_feedback_command command", &msg);

    let mut message = bot.send_message(msg.chat.id, " ");
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}