use std::str::FromStr;

use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::{ParseMode, ReplyMarkup};
use teloxide::utils::command::parse_command;
use uuid::Uuid;

use resonanse_common::EventSubjectFilter;

use crate::ACCOUNTS_REPOSITORY;
use crate::data_translators::fill_base_account_from_teloxide_user;
use crate::handlers::{FillingEvent, HandlerResult, log_request, MyDialogue};
use crate::high_logics::send_event_post;
use crate::keyboards::get_inline_kb_set_subject_filter;
use crate::states::{BaseState, CreateEventState};

const BOT_HELP_TEXT_MD: &str = "Помощ";
const CREATE_EVENT_TEXT_MD: &str = "Введите название события";
const GET_EVENTS_TEXT_MD: &str = "Выбери, что тебе интересно";

pub async fn start_command(bot: Bot, msg: Message) -> HandlerResult {
    log_request("got start_command", &msg);

    if let Some(command_text) = msg.text() {
        if let Some((command, params)) = parse_command(command_text, "") {
            if let Some(first_param) = params.first() {
                if let Some(event_uuid) = first_param.strip_prefix("event_") {
                    if let Ok(event_uuid) = Uuid::from_str(event_uuid) {
                        send_event_post(&bot, msg.chat.id, event_uuid).await?;
                        return Ok(());
                    }
                }
            }
        }
    }

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

    dialogue.update(
        BaseState::CreateEvent {
            state: CreateEventState::Name,
            filling_event: FillingEvent::new(),
        }
    ).await?;

    Ok(())
}

pub async fn get_events_command(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log_request("got get_events command", &msg);

    const DEFAULT_PAGE_SIZE: i64 = 10;

    let (page, page_size) = (0i64, DEFAULT_PAGE_SIZE);

    let events_filter = EventSubjectFilter::new();

    dialogue.update(
        BaseState::GetEventList {
            page_size,
            page_num: page,
            events_filter: events_filter.clone(),
        }
    ).await?;

    let mut message = bot.send_message(msg.chat.id, GET_EVENTS_TEXT_MD);
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.reply_markup = Some(ReplyMarkup::InlineKeyboard(get_inline_kb_set_subject_filter(&events_filter)));
    message.await?;

    Ok(())
}

pub async fn send_feedback_command(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    log_request("got send_feedback_command command", &msg);

    let mut message = bot.send_message(msg.chat.id, "Введите отзыв, предложение или другую полезную обратную связь");
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    dialogue.update(
        BaseState::SendFeedback
    ).await?;

    Ok(())
}
