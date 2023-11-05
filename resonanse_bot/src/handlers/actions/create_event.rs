use std::ops::RangeInclusive;

use chrono::NaiveDateTime;
use log::{debug, warn};
use teloxide::prelude::*;
use teloxide::types::MessageKind::Common;
use teloxide::types::{
    InputFile, MediaKind, MediaLocation, MediaVenue, MessageCommon, ParseMode, ReplyMarkup,
};
use teloxide::utils::markdown;
use teloxide::Bot;
use uuid::Uuid;

use resonanse_common::file_storage::get_event_image_path_by_uuid;
use resonanse_common::models::{EventSubject, Location};
use resonanse_common::repository::CreateBaseEvent;

use crate::config::DEFAULT_DATETIME_FORMAT;
use crate::errors::BotHandlerError;
use crate::handlers::utils::download_file_by_id;
use crate::handlers::{log_request, HandlerResult, MyDialogue};
use crate::high_logics::publish_event;
use crate::keyboards;
use crate::keyboards::{get_inline_kb_choose_subject, get_inline_kb_edit_new_event};
use crate::states::{BaseState, CreateEventState};
use crate::utils::build_event_deep_link;

const DATETIME_FORMAT_1: &str = "%d/%m/%Y %H:%M";
const DATETIME_FORMAT_2: &str = "%d.%m.%Y %H.%M";
const DATETIME_FORMAT_3: &str = "%d-%m-%Y %H:%M";

const TITLE_LIMIT: RangeInclusive<usize> = 5..=100;
const DESCRIPTION_LIMIT: RangeInclusive<usize> = 15..=700;
const PLACE_TITLE_LIMIT: RangeInclusive<usize> = 0..=30;
const CONTACT_LIMIT: RangeInclusive<usize> = 3..=100;

#[derive(Clone, Default)]
pub struct FillingEvent {
    title: Option<String>,
    is_private: bool,
    subject: Option<EventSubject>,
    description: Option<String>,
    datetime: Option<chrono::NaiveDateTime>,
    geo_position: Option<Location>,
    picture: Option<Uuid>,
    contact_info: Option<String>,
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
            contact_info: None,
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

💡 Тематика: _{}_
📅 Дата: _{}_
{}
{}
"#,
            markdown::escape(&self.title),
            markdown::escape(&self.description),
            markdown::escape(&self.subject.to_string()),
            markdown::escape(&self.datetime.format(DEFAULT_DATETIME_FORMAT).to_string()),
            match &self.location.title.as_deref() {
                None => "".to_string(),
                Some(location_title) => format!("📍 Место: _{}_", markdown::escape(location_title)),
            },
            match &self.contact_info.as_deref() {
                None => "".to_string(),
                Some(contact_info) => format!("Контакт: _{}_", markdown::escape(contact_info)),
            },
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
            contact_info: value.contact_info,
        }
    }
}

macro_rules! reject_user_answer {
    ($bot: ident, $chat_id: expr, $text:expr) => {
        $bot.send_message($chat_id, $text).await?;
        return Ok(());
    };
}

macro_rules! check_msg_size {
    ($bot: ident, $chat_id: expr, $limit_range:ident, $value_to_check:ident) => {
        if $limit_range.contains(&$value_to_check.len()) {
            $value_to_check
        } else {
            $bot.send_message(
                $chat_id,
                format!(
                    "Количество символов ожидается от {} до {}. В вашем сообщении {}",
                    $limit_range.start(),
                    $limit_range.end(),
                    $value_to_check.len()
                ),
            ).await?;

            return Ok(());
        }
    };
}

pub async fn handle_create_event_state_message(
    bot: Bot,
    dialogue: MyDialogue,
    (create_event_state, filling_event): (CreateEventState, FillingEvent),
    msg: Message,
) -> HandlerResult {
    log_request(
        format!("handle_create_event_state_message {:?}", create_event_state),
        &msg,
    );

    match create_event_state {
        CreateEventState::Name => handle_event_name(bot, dialogue, msg, filling_event).await,
        // CreateEventState::Publicity => (),
        CreateEventState::Description => {
            handle_event_description(bot, dialogue, msg, filling_event).await
        }
        CreateEventState::Datetime => {
            handle_event_datetime(bot, dialogue, msg, filling_event).await
        }
        CreateEventState::Geo => handle_event_geo(bot, dialogue, msg, filling_event).await,
        CreateEventState::PlaceTitle => {
            handle_event_place_title(bot, dialogue, msg, filling_event).await
        }
        // CreateEventState::Subject => handle_event_subject,
        CreateEventState::Picture => handle_event_picture(bot, dialogue, msg, filling_event).await,
        CreateEventState::ContactInfo => {
            handle_event_contact(bot, dialogue, msg, filling_event).await
        }
        // CreateEventState::Finalisation => handle_event_finalisation(bot, dialogue, msg, filling_event).await,
        _ => {
            warn!(
                "Unhandled handle_create_event_state: {:?}",
                create_event_state
            );
            bot.send_message(msg.chat.id, "unknown create event handler")
                .await?;
            return Err(Box::try_from(BotHandlerError::UnknownHandler).unwrap());
        }
    }
}

pub async fn handle_create_event_state_callback(
    bot: Bot,
    dialogue: MyDialogue,
    (create_event_state, filling_event): (CreateEventState, FillingEvent),
    q: CallbackQuery,
) -> HandlerResult {
    let msg = match &q.message {
        None => {
            warn!("handle_create_event_state_callback without message");
            return Ok(());
        }
        Some(v) => v,
    };

    log_request(
        format!(
            "handle_create_event_state_callback {:?}",
            create_event_state
        ),
        &msg,
    );

    match create_event_state {
        // CreateEventState::Name => handle_event_name,
        // CreateEventState::Publicity => (),
        // CreateEventState::Description => handle_event_description,,
        // CreateEventState::Datetime => handle_event_datetime,
        // CreateEventState::Geo => handle_event_geo,
        CreateEventState::Subject => handle_event_subject(bot, dialogue, filling_event, q).await,
        // CreateEventState::Picture => handle_event_picture,
        CreateEventState::Finalisation => {
            handle_event_finalisation_callback(bot, dialogue, filling_event, q).await
        }
        _ => {
            warn!(
                "Unhandled handle_create_event_state_callback: {:?}",
                create_event_state
            );
            bot.send_message(msg.chat.id, "unknown create event handler")
                .await?;
            return Err(Box::try_from(BotHandlerError::UnknownHandler).unwrap());
        }
    }
}

pub async fn handle_event_name(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    mut filling_event: FillingEvent,
) -> HandlerResult {
    let event_name = match msg.text() {
        None => {
            reject_user_answer!(bot, msg.chat.id, "No name provided");
        }
        Some(v) => check_msg_size!(bot, msg.chat.id, TITLE_LIMIT, v).replace("\n", " "),
    };

    filling_event.title = Some(event_name.to_string());

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::Description,
            filling_event,
        })
        .await?;

    let message = bot.send_message(msg.chat.id, "Введите описание");
    message.await?;

    Ok(())
}

pub async fn handle_event_description(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    mut filling_event: FillingEvent,
) -> HandlerResult {
    let event_description = match msg.text() {
        None => {
            reject_user_answer!(bot, msg.chat.id, "No description provided");
        }
        Some(v) => check_msg_size!(bot, msg.chat.id, DESCRIPTION_LIMIT, v),
    };

    filling_event.description = Some(event_description.to_string());

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::Datetime,
            filling_event,
        })
        .await?;

    let current_date = chrono::offset::Local::now()
        .format(&markdown::escape(DEFAULT_DATETIME_FORMAT))
        .to_string();
    let mut message = bot.send_message(
        msg.chat.id,
        format!(
            "Введите дату и время в формате дд\\.мм\\.гггг чч:мм\\. Например: `{}`",
            current_date
        ),
    );
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}

pub async fn handle_event_datetime(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    mut filling_event: FillingEvent,
) -> HandlerResult {
    let event_dt = match msg.text() {
        None => {
            reject_user_answer!(bot, msg.chat.id, "No datetime provided");
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
            bot.send_message(msg.chat.id, "Дата и время не распознаны")
                .await?;
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
        "Отправьте ссылку в Yandex.Map или геометку (Прикрепить вложение -> локация)",
    );
    message.await?;

    Ok(())
}

pub async fn handle_event_geo(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    mut filling_event: FillingEvent,
) -> HandlerResult {
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
        Common(MessageCommon {
            media_kind: MediaKind::Text(media_text),
            ..
        }) => {
            let plain_text = media_text.text;

            match Location::parse_from_yandex_map_link(&plain_text) {
                Some(loc) => loc,
                None => {
                    reject_user_answer!(bot, msg.chat.id, "Место не распознано");
                }
            }
        }
        _ => {
            reject_user_answer!(bot, msg.chat.id, "No location provided");
        }
    };

    filling_event.geo_position = Some(location);

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::PlaceTitle,
            filling_event,
        })
        .await?;

    let message = bot.send_message(msg.chat.id, "Введите название места");
    message.await?;

    Ok(())
}

pub async fn handle_event_place_title(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    mut filling_event: FillingEvent,
) -> HandlerResult {
    debug!("provided msg: {:?}", msg);
    let place_title = match msg.text() {
        Some(place_title) => check_msg_size!(bot, msg.chat.id, PLACE_TITLE_LIMIT, place_title),
        _ => {
            reject_user_answer!(bot, msg.chat.id, "No location provided");
        }
    };

    filling_event
        .geo_position
        .as_mut()
        .map(|location| location.title = Some(place_title.to_string()));

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::Subject,
            filling_event,
        })
        .await?;

    let mut message = bot.send_message(msg.chat.id, "Выберите тематику");
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
        }
        Some(v) => EventSubject::from(v.as_ref()),
    };

    filling_event.subject = Some(event_subject);

    if let Some(msg) = q.message {
        bot.delete_message(q.from.id, msg.id).await?;
    }

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::Picture,
            filling_event,
        })
        .await?;

    let message = bot.send_message(
        q.from.id,
        "Осталось пара шагов. Добавьте изображение или постер",
    );
    message.await?;

    Ok(())
}

pub async fn handle_event_picture(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    mut filling_event: FillingEvent,
) -> HandlerResult {
    let event_photo_file_id = match msg.photo().and_then(|p| p.last()) {
        None => {
            reject_user_answer!(bot, msg.chat.id, "No photo provided");
        }
        Some(v) => v.file.id.clone(),
    };

    let local_file_uuid = Uuid::new_v4();
    let local_file_path = get_event_image_path_by_uuid(local_file_uuid);

    download_file_by_id(&bot, &event_photo_file_id, &local_file_path).await?;

    filling_event.picture = Some(local_file_uuid);

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::ContactInfo,
            filling_event: filling_event.clone(),
        })
        .await?;

    let message = bot.send_message(
        msg.chat.id,
        "Укажите контакт для связи. Например, юзернейм (как @resonanse_app)",
    );
    message.await?;

    Ok(())
}

pub async fn handle_event_contact(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    mut filling_event: FillingEvent,
) -> HandlerResult {
    let contact_info = match msg.text() {
        None => {
            reject_user_answer!(bot, msg.chat.id, "No subject provided");
        }
        Some(v) => check_msg_size!(bot, msg.chat.id, CONTACT_LIMIT, v).to_string(),
    };

    filling_event.contact_info = Some(contact_info);

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::Finalisation,
            filling_event: filling_event.clone(),
        })
        .await?;

    let local_file_path =
        get_event_image_path_by_uuid(filling_event.picture.ok_or("Picture not set")?);
    let mut message = bot.send_photo(msg.chat.id, InputFile::file(local_file_path));
    let event_text_representation = CreateBaseEvent::from(filling_event.clone()).format();
    let message_text = format!(
        "Готово, проверьте заполненные данные:\n {}",
        event_text_representation
    );
    message.caption = Some(message_text);
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.reply_markup = Some(ReplyMarkup::InlineKeyboard(get_inline_kb_edit_new_event(
        !filling_event.is_private,
        filling_event
            .geo_position
            .map(|geo| geo.get_yandex_map_link_to()),
    )));
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
        }
        Some(keyboards::EDIT_PUBLICITY_FALSE_CALLBACK) => {
            filling_event.is_private = true;
        }
        Some(keyboards::REFILL_EVENT_AGAIN_CALLBACK) => {
            bot.delete_message(msg.chat.id, msg.id).await?;
            dialogue
                .update(BaseState::CreateEvent {
                    state: CreateEventState::Name,
                    filling_event: FillingEvent::new(),
                })
                .await?;
            bot.send_message(
                msg.chat.id,
                "Хорошо, вы можете заполнить данные заново. Введите название",
            )
            .await?;
            return Ok(());
        }
        Some(keyboards::CREATE_EVENT_CALLBACK) => {
            bot.delete_message(msg.chat.id, msg.id).await?;
            dialogue.update(BaseState::Idle).await?;

            let tg_user = q.from;

            let created_event = publish_event(filling_event.clone(), &tg_user).await?;

            let tg_event_deep_link = build_event_deep_link(created_event.id);
            if filling_event.is_private {
                bot.send_message(
                    msg.chat.id,
                    format!(
                        "Событие создано. Оно будет доступно только по вашей ссылке: {}",
                        tg_event_deep_link
                    ),
                )
                .await?;
            } else {
                bot.send_message(
                    msg.chat.id,
                    format!(
                        "Событие опубликовано. Также вы можете поделиться им по ссылке: {}",
                        tg_event_deep_link
                    ),
                )
                .await?;
            }

            return Ok(());
        }
        _ => {
            bot.send_message(msg.chat.id, "Unknown callback finalisation action")
                .await?;
            return Ok(());
        }
    }

    {
        // update message anyway
        let mut edit_reply_markup = bot.edit_message_reply_markup(msg.chat.id, msg.id);
        edit_reply_markup.reply_markup = Some(get_inline_kb_edit_new_event(
            !filling_event.is_private,
            filling_event
                .geo_position
                .map(|geo| geo.get_yandex_map_link_to()),
        ));
        edit_reply_markup.await?;
    }

    Ok(())
}
