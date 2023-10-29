use std::fmt::format;
use std::fs::File;
use chrono::{DateTime, NaiveDateTime, ParseResult};
use log::{debug, warn};
use teloxide::Bot;
use teloxide::payloads::SendMessage;
use teloxide::prelude::*;
use teloxide::requests::JsonRequest;
use teloxide::types::{InlineKeyboardMarkup, InputFile, MediaKind, MediaLocation, MediaVenue, MessageCommon, ParseMode, PhotoSize, ReplyMarkup, Venue};
use teloxide::types::MessageKind::Common;
use teloxide::utils::markdown;
use uuid::Uuid;
use resonanse_common::file_storage::{get_event_image_path_by_uuid, get_event_images_path};
use resonanse_common::models::{BaseEvent, EventSubject, Location};
use resonanse_common::repository;
use resonanse_common::repository::CreateBaseEvent;
use crate::errors::BotHandlerError;
use crate::handlers::{HandlerResult, log_request, MyDialogue};
use crate::handlers::utils::download_file_by_id;
use crate::{ACCOUNTS_REPOSITORY, EVENTS_REPOSITORY, keyboards, MANAGER_BOT};
use crate::config::DEFAULT_DATETIME_FORMAT;
use crate::data_translators::fill_base_account_from_teloxide_user;
use crate::high_logics::publish_event;
use crate::keyboards::{get_inline_kb_choose_subject, get_inline_kb_edit_new_event};
use crate::states::{BaseState, CreateEventState};
use crate::utils::{build_event_deep_link, get_tg_downloads_dir, repr_user_as_str};

const DATETIME_FORMAT_1: &str = "%d/%m/%Y %H:%M";
const DATETIME_FORMAT_2: &str = "%d.%m.%Y %H.%M";
const DATETIME_FORMAT_3: &str = "%d-%m-%Y %H:%M";

#[derive(Clone, Default)]
pub struct FillingEvent {
    title: Option<String>,
    is_private: bool,
    subject: Option<EventSubject>,
    description: Option<String>,
    datetime: Option<chrono::NaiveDateTime>,
    geo_position: Option<Location>,
    picture: Option<Uuid>,
    creator_id: i64,
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
            picture: None,
            creator_id: 0,
        }
    }
}

pub trait TgTextFormatter {
    fn format(&self) -> String;
}

impl TgTextFormatter for CreateBaseEvent {
    fn format(&self) -> String {
        let msg_text = format!(
            r#"
*{}*
{}

Тематика: _{}_
Дата: _{}_
"#
            ,
            markdown::escape(&self.title),
            markdown::escape(&self.description),
            markdown::escape(&self.subject.to_string()),
            markdown::escape(&self.datetime.format(DEFAULT_DATETIME_FORMAT).to_string()),
            // markdown::escape(&self.location.get_yandex_map_link_to()),
        );

        msg_text
    }
}

impl From<FillingEvent> for CreateBaseEvent {
    fn from(value: FillingEvent) -> Self {
        CreateBaseEvent {
            // id: 0,
            is_private: false,
            is_commercial: false,
            title: value.title.unwrap_or("No title".to_string()),
            description: value.description.unwrap_or("No description".to_string()),
            subject: value.subject.unwrap_or(EventSubject::Other),
            datetime: value.datetime.unwrap_or(chrono::Local::now().naive_local()),
            // timezone: chrono_tz::Tz::Europe__Moscow,
            location: value.geo_position.unwrap_or_else(|| {
                warn!("cannot set event for BaseEvent from FillingEvent");
                Location::from_ll(0.0, 0.0)
            }),
            creator_id: value.creator_id,
            event_type: Default::default(),
            picture: value.picture,
            // creation_time: Default::default(),
        }
    }
}


macro_rules! reject_user_answer {
    ($bot: ident, $chat_id: expr, $text:expr) => {
        $bot.send_message(
            $chat_id,
            $text,
        ).await?;
        // debug!("rejected user ({}) answer: {}", repr_user_as_str($msg.from()), $text);
        return Ok(());
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
        // CreateEventState::Finalisation => handle_event_finalisation(bot, dialogue, msg, filling_event).await,
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


    match create_event_state {
        // CreateEventState::Name => handle_event_name,
        // CreateEventState::Publicity => (),
        // CreateEventState::Description => handle_event_description,,
        // CreateEventState::Datetime => handle_event_datetime,
        // CreateEventState::Geo => handle_event_geo,
        CreateEventState::Subject => handle_event_subject(bot, dialogue, filling_event, q).await,
        // CreateEventState::Picture => handle_event_picture,
        CreateEventState::Finalisation => handle_event_finalisation_callback(bot, dialogue, filling_event, q).await,
        _ => {
            warn!("Unhandled handle_create_event_state_callback: {:?}", create_event_state);
            bot.send_message(
                msg.chat.id,
                "unknown create event handler",
            ).await?;
            return Err(Box::try_from(BotHandlerError::UnknownHandler).unwrap());
        }
    }
}

pub async fn handle_event_name(bot: Bot, dialogue: MyDialogue, msg: Message, mut filling_event: FillingEvent) -> HandlerResult {
    let event_name = match msg.text() {
        None => {
            // bot.send_message(
            //     msg.chat.id,
            //     "No name provided",
            // ).await?;
            // return Ok(());
            reject_user_answer!(bot, msg.chat.id, "No name provided");
            // debug!("rejected user ({}) answer: {}", repr_user_as_str(msg.from()), $text);
            // msg.from()
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
            reject_user_answer!(bot, msg.chat.id, "No description provided");
            // bot.send_message(
            //     msg.chat.id,
            //     "No description provided",
            // ).await?;
            // return Ok(());
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
            reject_user_answer!(bot, msg.chat.id, "No datetime provided");
            // bot.send_message(
            //     msg.chat.id,
            //     "No datetime provided",
            // ).await?;
            // return Ok(());
        }
        Some(v) => v,
    };

    let event_dt = match NaiveDateTime::parse_from_str(event_dt, DEFAULT_DATETIME_FORMAT)
        .or(NaiveDateTime::parse_from_str(event_dt, DATETIME_FORMAT_1))
        .or(NaiveDateTime::parse_from_str(event_dt, DATETIME_FORMAT_2))
        .or(NaiveDateTime::parse_from_str(event_dt, DATETIME_FORMAT_3))
    {
        Ok(v) => v,
        Err(err) => {
            warn!("handle_event_datetime: parse date error {}", err);
            bot.send_message(
                msg.chat.id,
                "Дата и время не распознаны",
            ).await?;
            return Err(Box::new(err));
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
    // let event_location = match msg.location() {
    //     None => {
    //         debug!("provided msg: {:?}", msg);
    //         debug!("rejected user ({})", repr_user_as_str(msg.from()));
    //         reject_user_answer!(bot, msg.chat.id, "No location provided");
    //         // bot.send_message(
    //         //     msg.chat.id,
    //         //     "",
    //         // ).await?;
    //         // return Ok(());
    //     }
    //     Some(v) => v,
    // };

    debug!("provided msg: {:?}", msg);
    let location = match msg.kind {
        Common(MessageCommon {
                   media_kind: MediaKind::Location(MediaLocation { location, .. }),
                   ..
               }) => Location::from_ll(location.latitude, location.longitude),
        Common(MessageCommon {
                   media_kind: MediaKind::Venue(MediaVenue { venue, .. }),
                   ..
               }) => Location {
            latitude: venue.location.latitude,
            longitude: venue.location.longitude,
            title: Some(venue.title),
        },
        _ => {
            reject_user_answer!(bot, msg.chat.id, "No location provided");
        }
    };

    // let event_location = match msg.location() {
    //     None => {
    //         debug!("rejected user ({})", repr_user_as_str(msg.from()));
    //         reject_user_answer!(bot, msg.chat.id, "No location provided");
    //         // bot.send_message(
    //         //     msg.chat.id,
    //         //     "",
    //         // ).await?;
    //         // return Ok(());
    //     }
    //     Some(v) => v,
    // };

    filling_event.geo_position = Some(location);

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
            reject_user_answer!(bot, q.from.id, "No subject provided");
            // bot.send_message(
            //     q.from.id,
            //     "No subject provided",
            // ).await?;
            // return Ok(());
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
            reject_user_answer!(bot, msg.chat.id, "No photo provided");
            // bot.send_message(
            //     msg.chat.id,
            //     "No photo provided",
            // ).await?;
            // return Ok(());
        }
        Some(v) => {
            v.file.id.clone()
        }
    };

    let local_file_uuid = Uuid::new_v4();
    // let local_file_path = get_event_images_path().join(&local_file_uuid.to_string());
    let local_file_path = get_event_image_path_by_uuid(local_file_uuid);

    download_file_by_id(&bot, &event_photo_file_id, &local_file_path).await?;

    filling_event.picture = Some(local_file_uuid);

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::Finalisation,
            filling_event: filling_event.clone(),
        })
        .await?;

    let mut message = bot.send_photo(msg.chat.id, InputFile::file(local_file_path));
    let event_text_representation = CreateBaseEvent::from(filling_event.clone()).format();
    let message_text = format!("Готово, проверьте заполненные данные:\n {}", event_text_representation);
    message.caption = Some(message_text);
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.reply_markup = Some(ReplyMarkup::InlineKeyboard(get_inline_kb_edit_new_event(!filling_event.is_private, filling_event.geo_position.map(|geo| geo.get_yandex_map_link_to()))));
    message.await?;

    Ok(())
}


pub async fn handle_event_finalisation_callback(
    bot: Bot,
    dialogue: MyDialogue,
    mut filling_event: FillingEvent,
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(q.id.clone()).await?;
    let msg = match q.message {
        None => {
            warn!("No callback data for callback {}", q.id);
            return Ok(());
        }
        Some(v) => v,
    };

    match q.data.as_deref() {
        Some(keyboards::EDIT_PUBLICITY_TRUE_CALLBACK) => {
            filling_event.is_private = false;
            // update_reply_kb().await;
        }
        Some(keyboards::EDIT_PUBLICITY_FALSE_CALLBACK) => {
            filling_event.is_private = true;
            // update_reply_kb().await;
        }
        Some(keyboards::REFILL_EVENT_AGAIN_CALLBACK) => {
            bot.delete_message(msg.chat.id, msg.id).await?;
            dialogue
                .update(BaseState::CreateEvent {
                    state: CreateEventState::Name,
                    filling_event: FillingEvent::new(),
                })
                .await?;
            bot.send_message(msg.chat.id, "Хорошо, вы можете заполнить данные заново. Введите название").await?;
            return Ok(());
        }
        Some(keyboards::CREATE_EVENT_CALLBACK) => {
            bot.delete_message(msg.chat.id, msg.id).await?;
            dialogue
                .update(BaseState::Idle)
                .await?;

            let tg_user = q.from;

            let created_event= publish_event(filling_event.clone(), &tg_user).await?;

            let tg_event_deep_link = build_event_deep_link(
                created_event.id
            );
            if filling_event.is_private {
                bot.send_message(
                    msg.chat.id,
                    format!(
                        "Событие создано. Оно будет доступно только по вашей ссылке: {}",
                        tg_event_deep_link
                    )
                ).await?;
            } else {
                 bot.send_message(
                    msg.chat.id,
                    format!(
                        "Событие опубликовано. Также вы можете поделиться им по ссылке: {}",
                        tg_event_deep_link
                    )
                ).await?;
            }

            return Ok(());
        }
        _ => {
            bot.send_message(
                msg.chat.id,
                "Unknown callback finalisation action",
            ).await?;
            return Ok(());
        }
    }

    {
        // update message anyway
        let mut edit_reply_markup = bot.edit_message_reply_markup(
            msg.chat.id, msg.id,
        );
        edit_reply_markup.reply_markup = Some(get_inline_kb_edit_new_event(!filling_event.is_private, filling_event.geo_position.map(|geo| geo.get_yandex_map_link_to())));
        edit_reply_markup.await?;
    }

    Ok(())
}
