use std::ops::RangeInclusive;
use std::str::FromStr;

use chrono::NaiveDateTime;
use log::{debug, warn};
use teloxide::prelude::*;
use teloxide::types::MessageKind::Common;
use teloxide::types::ParseMode::MarkdownV2;
use teloxide::types::{
    MediaKind, MediaLocation, MediaVenue, MessageCommon, MessageId, ParseMode, ReplyMarkup,
};
use teloxide::utils::markdown;
use teloxide::Bot;
use uuid::Uuid;

use resonanse_common::file_storage::get_event_image_path_by_uuid;
use resonanse_common::models::{BaseEvent, EventSubject, Location, ResonanseEventKind};

use crate::config::DEFAULT_DATETIME_FORMAT;
use crate::data_structs::{
    prepare_event_msg_with_base_event, EventPostMessageRequest, FillingEvent,
};
use crate::errors::BotHandlerError;
use crate::handlers::utils::download_file_by_id;
use crate::handlers::{HandlerResult, MyDialogue};
use crate::high_logics::publish_event;
use crate::keyboards;
use crate::keyboards::{get_inline_kb_choose_event_kind, get_make_event_keyboard};
use crate::states::{BaseState, CreateEventState};
use crate::utils::build_event_deep_link;

const DATETIME_FORMAT_1: &str = "%d/%m/%Y %H:%M";
const DATETIME_FORMAT_2: &str = "%d.%m.%Y %H.%M";
const DATETIME_FORMAT_3: &str = "%d-%m-%Y %H:%M";

const TITLE_LIMIT: RangeInclusive<usize> = 5..=100;
const DESCRIPTION_LIMIT: RangeInclusive<usize> = 15..=764;
const PLACE_TITLE_LIMIT: RangeInclusive<usize> = 0..=40;
const CONTACT_LIMIT: RangeInclusive<usize> = 3..=40;

macro_rules! reject_user_answer {
    ($bot: ident, $chat_id: expr, $text:expr) => {
        $bot.send_message($chat_id, $text).await?;
        // debug!("rejected user ({}) answer: {}", repr_user_as_str($msg.from()), $text);
        return Err(Box::new(BotHandlerError::UserInputRejected));
    };
}
macro_rules! check_msg_size {
    ($bot: ident, $chat_id: expr, $limit_range:ident, $value_to_check:ident) => {
        if $limit_range.contains(&$value_to_check.chars().count()) {
            $value_to_check
        } else {
            $bot.send_message(
                $chat_id,
                format!(
                    "Количество символов ожидается от {} до {}. В вашем сообщении {}",
                    $limit_range.start(),
                    $limit_range.end(),
                    $value_to_check.chars().count()
                ),
            ).await?;

            return Err(Box::new(BotHandlerError::UserInputRejected));
        }
    };
}

pub async fn handle_fill_event_field_callback(
    bot: Bot,
    dialogue: MyDialogue,
    filling_event: FillingEvent,
    last_edit_msg_id: MessageId,
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(q.id.clone()).await?;

    let (fill_field_state, msg_text, reply_markup) = match q.data.as_deref() {
        Some(keyboards::FILL_EVENT_TITLE_BTN_ID) => (
            CreateEventState::EventTitle,
            t!("actions.create_event.fill_event.event_title"),
            None,
        ),
        Some(keyboards::FILL_EVENT_SUBJECT_BTN_ID) => (
            CreateEventState::Subject,
            t!("actions.create_event.fill_event.subject"),
            Some(keyboards::get_inline_kb_choose_subject()),
        ),
        Some(keyboards::FILL_EVENT_DESCRIPTION_BTN_ID) => (
            CreateEventState::Description,
            t!("actions.create_event.fill_event.description"),
            None,
        ),

        Some(keyboards::FILL_EVENT_DATETIME_FROM_BTN_ID) => {
            let cur_dt = chrono::offset::Local::now()
                .naive_local()
                .format(&markdown::escape(DEFAULT_DATETIME_FORMAT))
                .to_string();

            (
                CreateEventState::DatetimeFrom,
                t!(
                    "actions.create_event.fill_event.datetime_from",
                    dt_example = cur_dt
                ),
                None,
            )
        }
        Some(keyboards::FILL_EVENT_DATETIME_TO_BTN_ID) => {
            let cur_dt = chrono::offset::Local::now()
                .naive_local()
                .format(&markdown::escape(DEFAULT_DATETIME_FORMAT))
                .to_string();

            (
                CreateEventState::DatetimeTo,
                t!(
                    "actions.create_event.fill_event.datetime_from",
                    dt_example = cur_dt
                ),
                None,
            )
        }

        Some(keyboards::FILL_EVENT_LOCATION_GEO_BTN_ID) => (
            CreateEventState::Geo,
            t!("actions.create_event.fill_event.geo"),
            None,
        ),
        Some(keyboards::FILL_EVENT_LOCATION_TITLE_BTN_ID) => (
            CreateEventState::PlaceTitle,
            t!("actions.create_event.fill_event.location_title"),
            None,
        ),
        Some(keyboards::FILL_EVENT_PICTURE_BTN_ID) => (
            CreateEventState::Picture,
            t!("actions.create_event.fill_event.picture"),
            None,
        ),
        Some(keyboards::FILL_EVENT_CONTACT_BTN_ID) => (
            CreateEventState::ContactInfo,
            t!("actions.create_event.fill_event.contact"),
            None,
        ),
        Some(keyboards::FILL_EVENT_KIND_BTN_ID) => (
            CreateEventState::EventKind,
            t!("actions.create_event.fill_event.event_kind"),
            Some(get_inline_kb_choose_event_kind()),
        ),
        Some(keyboards::FILL_EVENT_FINALIZE_BTN_ID) => {
            return handle_event_finalisation_callback(bot, dialogue, filling_event, q).await
        }
        // CreateEventState::Finalisation,
        // t!("actions.create_event.fill_event.finalize"),
        // None,
        _ => {
            warn!(
                "handle_fill_event_field_callback unknown callback data: {:?}",
                q.data
            );
            // bot.send_message(q.from.id, "unknown create event handler")
            //     .await?;
            return Err(Box::new(BotHandlerError::UnknownHandler));
        }
    };

    dialogue
        .update(BaseState::CreateEvent {
            state: fill_field_state,
            filling_event,
            last_edit_msg_id,
        })
        .await?;

    if reply_markup.is_some() {
        if let Some(msg) = q.message {
            let mut edit_message = bot.edit_message_reply_markup(q.from.id, msg.id);
            edit_message.reply_markup = reply_markup;
            edit_message.await?;
        }
    } else {
        let mut message = bot.send_message(q.from.id, msg_text);
        message.parse_mode = Some(MarkdownV2);
        // message.reply_markup = reply_markup;
        message.await?;
    }

    Ok(())
}

pub async fn handle_create_event_state_message(
    bot: Bot,
    dialogue: MyDialogue,
    (create_event_state, mut filling_event, last_edit_msg_id): (
        CreateEventState,
        FillingEvent,
        MessageId,
    ),
    msg: Message,
) -> HandlerResult {
    let chat_id = msg.chat.id;
    match create_event_state {
        CreateEventState::Idle => {
            bot.send_message(msg.chat.id, "Вы не нажали, что именно хотите заполнить")
                .await?;
            return Ok(());
        }
        CreateEventState::EventTitle => handle_event_name(&bot, msg, &mut filling_event).await?,
        // CreateEventState::Publicity => (),
        CreateEventState::Description => {
            handle_event_description(&bot, msg, &mut filling_event).await?
        }
        CreateEventState::DatetimeFrom => {
            handle_event_datetime(&bot, msg, &mut filling_event.datetime_from).await?
        }
        CreateEventState::DatetimeTo => {
            handle_event_datetime(&bot, msg, &mut filling_event.datetime_to).await?
        }
        CreateEventState::Geo => handle_event_geo(&bot, msg, &mut filling_event).await?,
        CreateEventState::PlaceTitle => {
            handle_event_place_title(&bot, msg, &mut filling_event).await?
        }
        // CreateEventState::Subject => handle_event_subject,
        CreateEventState::Picture => handle_event_picture(&bot, msg, &mut filling_event).await?,
        CreateEventState::ContactInfo => {
            handle_event_contact(&bot, msg, &mut filling_event).await?
        }
        // CreateEventState::Finalisation => handle_event_finalisation(bot, dialogue, msg, filling_event).await,
        _ => {
            warn!(
                "Unhandled handle_create_event_state: {:?}",
                create_event_state
            );
            bot.send_message(msg.chat.id, "unknown create event handler")
                .await?;
            return Err(Box::new(BotHandlerError::UnknownHandler));
        }
    }

    update_filling_message(&bot, dialogue, filling_event, chat_id, last_edit_msg_id).await?;

    Ok(())

    // handler(bot, dialogue, msg, filling_event)
}

pub async fn handle_create_event_state_callback(
    bot: Bot,
    dialogue: MyDialogue,
    (create_event_state, mut filling_event, last_edit_msg_id): (
        CreateEventState,
        FillingEvent,
        MessageId,
    ),
    q: CallbackQuery,
) -> HandlerResult {
    let msg = match &q.message {
        None => {
            warn!("handle_create_event_state_callback without message");
            return Ok(());
        }
        Some(v) => v,
    };
    let chat_id = msg.chat.id;

    match handle_fill_event_field_callback(
        bot.clone(),
        dialogue.clone(),
        filling_event.clone(),
        last_edit_msg_id,
        q.clone(),
    )
    .await
    {
        Ok(v) => return Ok(v),
        Err(err) => {
            warn!("handle_fill_event_field_callback error: {:?}", err);
        }
    }

    match create_event_state {
        // CreateEventState::Idle => {
        //     return handle_fill_event_field_callback(bot, dialogue, filling_event, q).await;
        // }
        // CreateEventState::Name => handle_event_name,
        // CreateEventState::Publicity => (),
        // CreateEventState::Description => handle_event_description,,
        // CreateEventState::Datetime => handle_event_datetime,
        // CreateEventState::Geo => handle_event_geo,
        CreateEventState::Subject => handle_event_subject(&bot, &mut filling_event, q).await?,
        // CreateEventState::Picture => handle_event_picture,
        // CreateEventState::Finalisation => {
        //     handle_event_finalisation_callback(bot, dialogue, filling_event, q).await
        // }
        CreateEventState::EventKind => handle_event_kind(&bot, &mut filling_event, q).await?,
        _ => {
            // return handle_fill_event_field_callback(
            //     bot, dialogue, filling_event,
            //     last_edit_msg_id,
            //     q,
            // ).await;
            warn!(
                "Unhandled handle_create_event_state_callback: {:?}",
                create_event_state
            );
            bot.send_message(msg.chat.id, "unknown create event handler")
                .await?;
            return Err(Box::new(BotHandlerError::UnknownHandler));
        }
    }

    update_filling_message(&bot, dialogue, filling_event, chat_id, last_edit_msg_id).await?;

    // dialogue
    //     .update(BaseState::CreateEvent {
    //         state: CreateEventState::Idle,
    //         filling_event,
    //         last_edit_msg_id,
    //     })
    //     .await?;
    Ok(())
}

pub async fn update_filling_message(
    bot: &Bot,
    dialogue: MyDialogue,
    filling_event: FillingEvent,
    chat_id: ChatId,
    last_edit_msg_id: MessageId,
) -> HandlerResult {
    let sent_event_message: Message = match BaseEvent::try_from(filling_event.clone()) {
        Ok(base_event) if filling_event.is_ready() => {
            let event_message = prepare_event_msg_with_base_event(
                bot,
                chat_id,
                base_event,
                Some(ReplyMarkup::InlineKeyboard(
                    keyboards::get_make_event_keyboard(),
                )),
            );

            match event_message {
                EventPostMessageRequest::WithPoster(req) => req.await?,
                EventPostMessageRequest::Text(req) => req.await?,
            }
        }
        _ => {
            let missed_data_hint = filling_event.get_missed_data_hint();
            let mut message = bot.send_message(chat_id, missed_data_hint);
            message.parse_mode = Some(ParseMode::MarkdownV2);
            message.reply_markup = Some(ReplyMarkup::InlineKeyboard(get_make_event_keyboard()));
            message.await?
        }
    };

    match bot.delete_message(chat_id, last_edit_msg_id).await {
        Ok(_) => {}
        Err(err) => {
            debug!(
                "error on deleting message {} from chat {}: {:?}",
                last_edit_msg_id, chat_id, err
            );
        }
    };

    dialogue
        .update(BaseState::CreateEvent {
            state: CreateEventState::Idle,
            filling_event,
            last_edit_msg_id: sent_event_message.id,
        })
        .await?;

    Ok(())
}

pub async fn handle_event_name(
    bot: &Bot,
    // dialogue: MyDialogue,
    msg: Message,
    filling_event: &mut FillingEvent,
) -> HandlerResult {
    let event_name = match msg.text() {
        None => {
            reject_user_answer!(bot, msg.chat.id, "No name provided");
        }
        Some(v) => check_msg_size!(bot, msg.chat.id, TITLE_LIMIT, v).replace('\n', " "),
    };
    filling_event.title = Some(event_name.to_string());

    // dialogue
    //     .update(BaseState::CreateEvent {
    //         state: CreateEventState::Idle,
    //         filling_event,
    //     })
    //     .await?;

    Ok(())
}

pub async fn handle_event_description(
    bot: &Bot,
    // dialogue: MyDialogue,
    msg: Message,
    filling_event: &mut FillingEvent,
) -> HandlerResult {
    let event_description = match msg.text() {
        None => {
            reject_user_answer!(bot, msg.chat.id, "No description provided");
        }
        Some(v) => check_msg_size!(bot, msg.chat.id, DESCRIPTION_LIMIT, v),
    };

    filling_event.description = Some(event_description.to_string());

    // dialogue
    //     .update(BaseState::CreateEvent {
    //         state: CreateEventState::Idle,
    //         filling_event,
    //     })
    //     .await?;

    // let current_date = chrono::offset::Local::now()
    //     .format(&markdown::escape(DEFAULT_DATETIME_FORMAT))
    //     .to_string();
    // let mut message = bot.send_message(
    //     msg.chat.id,
    //     format!(
    //         "Введите дату и время в формате дд\\.мм\\.гггг чч:мм\\. Например: `{}`",
    //         current_date
    //     ),
    // );
    // message.parse_mode = Some(ParseMode::MarkdownV2);
    // message.await?;

    Ok(())
}

pub async fn handle_event_datetime(
    bot: &Bot,
    // dialogue: MyDialogue,
    msg: Message,
    // mut filling_event: FillingEvent,
    filling_dt: &mut Option<NaiveDateTime>,
) -> HandlerResult {
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
            bot.send_message(msg.chat.id, "Дата и время не распознаны")
                .await?;
            return Err(Box::new(err));
        }
    };

    *filling_dt = Some(event_dt);

    // dialogue
    //     .update(BaseState::CreateEvent {
    //         state: CreateEventState::Idle,
    //         filling_event,
    //     })
    //     .await?;
    //
    // let message = bot.send_message(
    //     msg.chat.id,
    //     "",
    // );
    // message.await?;

    Ok(())
}

pub async fn handle_event_geo(
    bot: &Bot,
    // dialogue: MyDialogue,
    msg: Message,
    filling_event: &mut FillingEvent,
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

    // dialogue
    //     .update(BaseState::CreateEvent {
    //         state: CreateEventState::Idle,
    //         filling_event,
    //     })
    //     .await?;
    //
    // let mut message = bot.send_message(msg.chat.id, "Выберите тематику");
    // message.reply_markup = Some(get_inline_kb_choose_subject());
    // message.await?;

    Ok(())
}

pub async fn handle_event_place_title(
    bot: &Bot,
    msg: Message,
    filling_event: &mut FillingEvent,
) -> HandlerResult {
    let place_title = match msg.text() {
        Some(place_title) => check_msg_size!(bot, msg.chat.id, PLACE_TITLE_LIMIT, place_title),
        _ => {
            reject_user_answer!(bot, msg.chat.id, "No location provided");
        }
    };

    filling_event.location_title = Some(place_title.to_string());

    Ok(())
}
//
// pub async fn handle_event_place_title(
//     bot: Bot,
//     // dialogue: MyDialogue,
//     msg: Message,
//     filling_event: &mut FillingEvent,
// ) -> HandlerResult {
//     debug!("provided msg: {:?}", msg);
//     let location = match msg.kind {
//         Common(MessageCommon {
//             media_kind: MediaKind::Location(MediaLocation { location, .. }),
//             ..
//         }) => Location::from_ll(location.latitude, location.longitude),
//         Common(MessageCommon {
//             media_kind: MediaKind::Venue(MediaVenue { venue, .. }),
//             ..
//         }) => {
//             filling_event.location_title = Some(venue.title);
//             Location {
//                 latitude: venue.location.latitude,
//                 longitude: venue.location.longitude,
//             }
//         }
//         _ => {
//             reject_user_answer!(bot, msg.chat.id, "No location provided");
//         }
//     };
//
//     filling_event.geo_position = Some(location);
//
//     // dialogue
//     //     .update(BaseState::CreateEvent {
//     //         state: CreateEventState::Subject,
//     //         filling_event,
//     //     })
//     //     .await?;
//     //
//     // let mut message = bot.send_message(msg.chat.id, "Выберите тематику");
//     // message.reply_markup = Some(get_inline_kb_choose_subject());
//     // message.await?;
//
//     Ok(())
// }

pub async fn handle_event_subject(
    bot: &Bot,
    // dialogue: MyDialogue,
    filling_event: &mut FillingEvent,
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(q.id).await?;
    let event_subject = match q.data.as_ref() {
        None => {
            reject_user_answer!(bot, q.from.id, "No subject provided");
        }
        Some(v) => EventSubject::from_str(v.as_ref())?,
    };

    filling_event.subject = Some(event_subject);

    if let Some(msg) = q.message {
        bot.delete_message(q.from.id, msg.id).await?;
    }

    // dialogue
    //     .update(BaseState::CreateEvent {
    //         state: CreateEventState::Picture,
    //         filling_event,
    //     })
    //     .await?;
    //
    // let message = bot.send_message(
    //     q.from.id,
    //     "Осталось пара шагов. Добавьте изображение или постер",
    // );
    // message.await?;

    Ok(())
}

pub async fn handle_event_kind(
    bot: &Bot,
    // dialogue: MyDialogue,
    filling_event: &mut FillingEvent,
    q: CallbackQuery,
) -> HandlerResult {
    bot.answer_callback_query(q.id).await?;
    let event_kind = match q.data.as_ref() {
        None => {
            reject_user_answer!(bot, q.from.id, "No subject provided");
        }
        Some(v) => ResonanseEventKind::from_str(v.as_ref())?,
    };

    filling_event.event_kind = event_kind;

    if let Some(msg) = q.message {
        bot.delete_message(q.from.id, msg.id).await?;
    }

    Ok(())
}

pub async fn handle_event_picture(
    bot: &Bot,
    // dialogue: MyDialogue,
    msg: Message,
    filling_event: &mut FillingEvent,
) -> HandlerResult {
    let event_photo_file_id = match msg.photo().and_then(|p| p.last()) {
        None => {
            reject_user_answer!(bot, msg.chat.id, "No photo provided");
        }
        Some(v) => v.file.id.clone(),
    };

    let local_file_uuid = Uuid::new_v4();
    let local_file_path = get_event_image_path_by_uuid(local_file_uuid);

    download_file_by_id(bot, &event_photo_file_id, &local_file_path).await?;

    filling_event.picture = Some(local_file_uuid);

    // dialogue
    //     .update(BaseState::CreateEvent {
    //         state: CreateEventState::ContactInfo,
    //         filling_event: filling_event.clone(),
    //     })
    //     .await?;
    //
    // let message = bot.send_message(
    //     msg.chat.id,
    //     "Укажите контакт для связи. Например, юзернейм (как @resonanse_app)",
    // );
    // message.await?;

    Ok(())
}

pub async fn handle_event_contact(
    bot: &Bot,
    // dialogue: MyDialogue,
    msg: Message,
    filling_event: &mut FillingEvent,
) -> HandlerResult {
    let contact_info = match msg.text() {
        None => {
            reject_user_answer!(bot, msg.chat.id, "No subject provided");
        }
        Some(v) => check_msg_size!(bot, msg.chat.id, CONTACT_LIMIT, v).to_string(),
    };

    filling_event.contact_info = Some(contact_info);

    // dialogue
    //     .update(BaseState::CreateEvent {
    //         state: CreateEventState::Finalisation,
    //         filling_event: filling_event.clone(),
    //     })
    //     .await?;

    // let local_file_path =
    //     get_event_image_path_by_uuid(filling_event.picture.ok_or("Picture not set")?);
    // let mut message = bot.send_photo(msg.chat.id, InputFile::file(local_file_path));
    // let event_text_representation = CreateBaseEvent::from(filling_event.clone()).format();
    // let message_text = format!(
    //     "Готово, проверьте заполненные данные:\n {}",
    //     event_text_representation
    // );
    // message.caption = Some(message_text);
    // message.parse_mode = Some(ParseMode::MarkdownV2);
    // message.reply_markup = Some(ReplyMarkup::InlineKeyboard(get_inline_kb_edit_new_event(
    //     !filling_event.is_private,
    //     filling_event
    //         .geo_position
    //         .map(|geo| geo.get_yandex_map_link_to()),
    // )));
    // message.await?;

    Ok(())
}

pub async fn handle_event_finalisation_callback(
    bot: Bot,
    dialogue: MyDialogue,
    filling_event: FillingEvent,
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

    let tg_user = q.from;

    let created_event = match publish_event(filling_event.clone(), &tg_user).await {
        Ok(v) => v,
        Err(err) => {
            bot.send_message(msg.chat.id, format!("Событие не создано. Ошибка: {}", err))
                .await?;

            return Ok(());
        }
    };
    bot.delete_message(msg.chat.id, msg.id).await?;
    dialogue.update(BaseState::Idle).await?;

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
            t!(
                "actions.create_event.fill_event.finalize_public",
                event_link = tg_event_deep_link
            ),
        )
        .await?;
    }

    Ok(())
}
