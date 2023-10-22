use std::fmt::format;
use std::fs::File;
use chrono::{DateTime, NaiveDateTime, ParseResult};
use log::warn;
use teloxide::Bot;
use teloxide::payloads::SendMessage;
use teloxide::prelude::*;
use teloxide::requests::JsonRequest;
use teloxide::types::{InputFile, ParseMode, PhotoSize};
use teloxide::utils::markdown;
use resonanse_common::models::{BaseEvent, EventSubject, Location};
use crate::errors::BotHandlerError;
use crate::handlers::{HandlerResult, log_request, MyDialogue};
use crate::handlers::utils::download_file_by_id;
use crate::keyboards::get_inline_kb_choose_subject;
use crate::states::{BaseState, CreateEventState};
use crate::utils::get_tg_downloads_dir;

const DEFAULT_DATETIME_FORMAT: &str = "%d.%m.%Y %H:%M";

#[derive(Clone, Default)]
pub struct FillingEvent {
    title: Option<String>,
    is_private: bool,
    subject: Option<EventSubject>,
    description: Option<String>,
    datetime: Option<chrono::NaiveDateTime>,
    geo_position: Option<Location>,
    photo: Option<String>,
}

impl FillingEvent {
    pub fn new() -> Self {
        FillingEvent {
            title: None,
            is_private: false,
            subject: None,
            description: None,
            datetime: None,
            geo_position: None,
            photo: None,
        }
    }
}

trait TgTextFormatter {
    fn format(&self) -> String;
}

impl TgTextFormatter for BaseEvent {
    fn format(&self) -> String {
        let msg_text = format!(
            r#"
**{}**
{}

Тематика: *{}*
Дата: *{}*
"#
            ,
            markdown::escape(&self.title),
            markdown::escape(&self.description),
            markdown::escape(&self.subject.to_string()),
            markdown::escape(&self.datetime.format(DEFAULT_DATETIME_FORMAT).to_string()),
        );

        msg_text
    }
}

impl From<FillingEvent> for BaseEvent {
    fn from(value: FillingEvent) -> Self {
        BaseEvent {
            id: 0,
            is_private: false,
            is_commercial: false,
            title: value.title.unwrap_or("No title".to_string()),
            description: value.description.unwrap_or("No description".to_string()),
            subject: value.subject.unwrap_or(EventSubject::Other),
            datetime: value.datetime.unwrap_or(chrono::Local::now().naive_local()),
            timezone: chrono_tz::Tz::Europe__Moscow,
            location: value.geo_position.unwrap_or(Location { latitude: 0.0, longitude: 0.0 }),
            creator_id: 0,
            event_type: Default::default(),
            picture: Default::default(),
        }
    }
}

pub async fn handle_create_event_state_message(bot: Bot, dialogue: MyDialogue, (create_event_state, filling_event): (CreateEventState, FillingEvent), msg: Message) -> HandlerResult {
    log_request(format!("handle_create_event_state_message {:?}", create_event_state), &msg);

    // type CreateEventMessageHandler = fn(Bot, MyDialogue, Message, FillingEvent) -> HandlerResult;

    match create_event_state {
        CreateEventState::Name => handle_event_name(bot, dialogue, msg, filling_event).await,
        // CreateEventState::Publicity => (),
        CreateEventState::Description => handle_event_description(bot, dialogue, msg, filling_event).await,
        CreateEventState::Datetime => handle_event_datetime(bot, dialogue, msg, filling_event).await,
        CreateEventState::Geo => handle_event_geo(bot, dialogue, msg, filling_event).await,
        // CreateEventState::Subject => handle_event_subject,
        CreateEventState::Picture => handle_event_picture(bot, dialogue, msg, filling_event).await,
        _ => {
            warn!("Unhandled handle_create_event_state: {:?}", create_event_state);
            bot.send_message(
                msg.chat.id,
                "unknown create event handler",
            ).await?;
            return Err(Box::try_from(BotHandlerError::UnknownHandler).unwrap());
        }
    }

    // handler(bot, dialogue, msg, filling_event)
}

pub async fn handle_create_event_state_callback(bot: Bot, dialogue: MyDialogue, (create_event_state, filling_event): (CreateEventState, FillingEvent), q: CallbackQuery) -> HandlerResult {
    let msg = match &q.message {
        None => {
            warn!("handle_create_event_state_callback without message");
            return Ok(());
        }
        Some(v) => v,
    };

    log_request(format!("handle_create_event_state_callback {:?}", create_event_state), &msg);


    let handler = match create_event_state {
        // CreateEventState::Name => handle_event_name,
        // CreateEventState::Publicity => (),
        // CreateEventState::Description => handle_event_description,,
        // CreateEventState::Datetime => handle_event_datetime,
        // CreateEventState::Geo => handle_event_geo,
        CreateEventState::Subject => handle_event_subject,
        // CreateEventState::Picture => handle_event_picture,
        _ => {
            warn!("Unhandled handle_create_event_state_callback: {:?}", create_event_state);
            bot.send_message(
                msg.chat.id,
                "unknown create event handler",
            ).await?;
            return Err(Box::try_from(BotHandlerError::UnknownHandler).unwrap());
        }
    };

    handler(bot, dialogue, filling_event, q).await
}

pub async fn handle_event_name(bot: Bot, dialogue: MyDialogue, msg: Message, mut filling_event: FillingEvent) -> HandlerResult {
    let event_name = match msg.text() {
        None => {
            bot.send_message(
                msg.chat.id,
                "No name provided",
            ).await?;
            return Ok(());
        }
        Some(v) => v,
    };

    filling_event.title = Some(event_name.to_string());

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::Description,
            filling_event,
        })
        .await?;

    let message = bot.send_message(
        msg.chat.id,
        "Введите описание",
    );
    message.await?;

    Ok(())
}

pub async fn handle_event_description(bot: Bot, dialogue: MyDialogue, msg: Message, mut filling_event: FillingEvent) -> HandlerResult {
    let event_description = match msg.text() {
        None => {
            bot.send_message(
                msg.chat.id,
                "No description provided",
            ).await?;
            return Ok(());
        }
        Some(v) => v,
    };

    filling_event.description = Some(event_description.to_string());

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::Datetime,
            filling_event,
        })
        .await?;

    let current_date = chrono::offset::Local::now().format(&markdown::escape(DEFAULT_DATETIME_FORMAT)).to_string();
    let mut message = bot.send_message(
        msg.chat.id,
        format!("Введите дату и время в формате дд\\.мм\\.гггг чч:мм\\. Например: `{}`", current_date),
    );
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}

pub async fn handle_event_datetime(bot: Bot, dialogue: MyDialogue, msg: Message, mut filling_event: FillingEvent) -> HandlerResult {
    let event_dt = match msg.text() {
        None => {
            bot.send_message(
                msg.chat.id,
                "No datetime provided",
            ).await?;
            return Ok(());
        }
        Some(v) => v,
    };

    let event_dt = match NaiveDateTime::parse_from_str(event_dt, DEFAULT_DATETIME_FORMAT) {
        Ok(v) => v,
        Err(err) => {
            warn!("handle_event_datetime: parse date error {}", err);
            return Err(Box::new(err))
        }
    };

    filling_event.datetime = Some(event_dt);

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::Geo,
            filling_event,
        })
        .await?;

    let message = bot.send_message(
        msg.chat.id,
        "Отправьте геометку (вложением)",
    );
    message.await?;

    Ok(())
}

pub async fn handle_event_geo(bot: Bot, dialogue: MyDialogue, msg: Message, mut filling_event: FillingEvent) -> HandlerResult {
    let event_location = match msg.location() {
        None => {
            bot.send_message(
                msg.chat.id,
                "No location provided",
            ).await?;
            return Ok(());
        }
        Some(v) => v,
    };

    filling_event.geo_position = Some(Location { latitude: event_location.latitude, longitude: event_location.longitude });

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::Subject,
            filling_event,
        })
        .await?;

    let mut message = bot.send_message(
        msg.chat.id,
        "Выберите тематику",
    );
    message.reply_markup = Some(get_inline_kb_choose_subject());
    message.await?;

    Ok(())
}

pub async fn handle_event_subject(
    bot: Bot,
    dialogue: MyDialogue,
    mut filling_event: FillingEvent,
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(q.id).await?;
    let event_subject = match q.data.as_ref() {
        None => {
            bot.send_message(
                q.from.id,
                "No subject provided",
            ).await?;
            return Ok(());
        }
        Some(v) => EventSubject::from(v.as_ref()),
    };

    filling_event.subject = Some(event_subject);

    if let Some(msg) = q.message {
        bot.delete_message(
            q.from.id,
            msg.id,
        ).await?;
    }

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::Picture,
            filling_event,
        })
        .await?;

    let mut message = bot.send_message(
        q.from.id,
        "Осталось добавить изображение",
    );
    message.await?;

    Ok(())
}

pub async fn handle_event_picture(bot: Bot, dialogue: MyDialogue, msg: Message, mut filling_event: FillingEvent) -> HandlerResult {
    let event_photo_file_id = match msg.photo().and_then(|p| p.last()) {
        None => {
            bot.send_message(
                msg.chat.id,
                "No photo provided",
            ).await?;
            return Ok(());
        }
        Some(v) => {
            v.file.id.clone()
        }
    };

    let local_file_uuid = uuid::Uuid::new_v4().to_string();
    let local_file_path = get_tg_downloads_dir().join(&local_file_uuid);

    download_file_by_id(&bot, &event_photo_file_id, &local_file_path).await?;

    filling_event.photo = Some(local_file_uuid);

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::Finalisation,
            filling_event: filling_event.clone(),
        })
        .await?;

    let mut message = bot.send_photo(msg.chat.id, InputFile::file(local_file_path));
    let event_text_representation = BaseEvent::from(filling_event).format();
    let message_text = format!("Готово, проверьте заполненные данные:\n {}", event_text_representation);
    message.caption = Some(message_text);

    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}