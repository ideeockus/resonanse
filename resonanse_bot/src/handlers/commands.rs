use std::env;
use std::str::FromStr;

use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::{ParseMode, ReplyMarkup};
use teloxide::utils::command::parse_command;
use uuid::Uuid;

use resonanse_common::EventSubjectFilter;

use crate::{ACCOUNTS_REPOSITORY, keyboards};
use crate::config::DONATION_URL;
use crate::data_structs::FillingEvent;
use crate::data_translators::fill_base_account_from_teloxide_user;
use crate::handlers::{HandlerResult, MyDialogue};
use crate::high_logics::send_event_post;
use crate::keyboards::{get_inline_kb_run_web_app, get_inline_kb_set_subject_filter};
use crate::states::{BaseState, CreateEventState};



pub async fn start_command(bot: Bot, msg: Message) -> HandlerResult {
    if let Some(command_text) = msg.text() {
        if let Some((_command, params)) = parse_command(command_text, "") {
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

    let mut message = bot.send_message(msg.chat.id, t!("hello_msg"));
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    if let Some(user) = msg.from() {
        let new_user_account = fill_base_account_from_teloxide_user(user);
        ACCOUNTS_REPOSITORY
            .get()
            .ok_or("Cannot get accounts repository")?
            .create_user_by_tg_user_id(new_user_account)
            .await?;
    }

    Ok(())
}

pub async fn help_command(bot: Bot, msg: Message) -> HandlerResult {
    let mut message = bot.send_message(msg.chat.id, t!("help_msg"));
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}

pub async fn create_event_command(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let mut message = bot.send_message(msg.chat.id, t!("actions.create_event.new_event"));
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    let filling_event = FillingEvent::new();

    let mut message = bot.send_message(msg.chat.id, filling_event.get_missed_data_hint());
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.reply_markup = Some(ReplyMarkup::InlineKeyboard(
        keyboards::get_make_event_keyboard(),
    ));
    let sent_msg: Message = message.await?;

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::Idle,
            filling_event,
            last_edit_msg_id: sent_msg.id,
        })
        .await?;

    Ok(())
}

pub async fn get_events_command(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    const DEFAULT_PAGE_SIZE: i64 = 10;

    let (page, page_size) = (0i64, DEFAULT_PAGE_SIZE);

    let events_filter = EventSubjectFilter::new();

    dialogue
        .update(BaseState::GetEventList {
            page_size,
            page_num: page,
            events_filter: events_filter.clone(),
        })
        .await?;

    let mut message = bot.send_message(msg.chat.id, t!("choose_category_msg"));
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.reply_markup = Some(ReplyMarkup::InlineKeyboard(
        get_inline_kb_set_subject_filter(&events_filter),
    ));
    message.await?;

    Ok(())
}


pub async fn set_user_city_command(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue
        .update(BaseState::SetCity)
        .await?;

    let mut message = bot.send_message(msg.chat.id, t!("actions.set_city.prompt"));
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}

pub async fn set_user_description_command(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    dialogue
        .update(BaseState::SetDescription)
        .await?;

    let mut message = bot.send_message(msg.chat.id, t!("actions.set_description.prompt"));
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}

pub async fn run_web_app_command(bot: Bot, msg: Message) -> HandlerResult {
    let mut message = bot.send_message(msg.chat.id, t!("choose_category_msg"));
    message.reply_markup = Some(ReplyMarkup::InlineKeyboard(get_inline_kb_run_web_app()));
    message.await?;

    Ok(())
}

pub async fn send_feedback_command(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let mut message = bot.send_message(msg.chat.id, t!("feedback_msg"));
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    dialogue.update(BaseState::SendFeedback).await?;

    Ok(())
}

pub async fn send_donation_command(bot: Bot, msg: Message) -> HandlerResult {
    let donation_url = env::var(DONATION_URL)?;
    let donation_msg = t!(
        "donation_msg",
        donation_link = &donation_url,
    );

    let message = bot.send_message(msg.chat.id, donation_msg);
    message.await?;

    Ok(())
}
