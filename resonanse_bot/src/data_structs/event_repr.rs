use chrono::NaiveDateTime;
use teloxide::payloads::{SendMessage, SendPhoto};
use teloxide::prelude::*;
use teloxide::requests::{JsonRequest, MultipartRequest};
use teloxide::types::{ChatId, InputFile, ParseMode, ReplyMarkup};
use teloxide::utils::markdown;
use teloxide::Bot;

use resonanse_common::file_storage::get_event_image_path_by_uuid;
use resonanse_common::models::BaseEvent;

use crate::config::DEFAULT_DATETIME_FORMAT;
use crate::keyboards::get_inline_kb_event_message;

pub enum EventPostMessageRequest {
    WithPoster(MultipartRequest<SendPhoto>),
    Text(JsonRequest<SendMessage>),
}

pub fn prepare_event_msg_with_base_event(
    bot: &Bot,
    chat_id: ChatId,
    base_event: BaseEvent,
    event_reply_markup: Option<ReplyMarkup>,
) -> EventPostMessageRequest {
    let formatted_data = match base_event.datetime_to {
        None => base_event
            .datetime_from
            .format(DEFAULT_DATETIME_FORMAT)
            .to_string(),
        Some(datetime_to) => {
            format!(
                "{} - {}",
                base_event
                    .datetime_from
                    .format(DEFAULT_DATETIME_FORMAT)
                    .to_string(),
                datetime_to.format(DEFAULT_DATETIME_FORMAT).to_string()
            )
        }
    };

    let msg_text = t!(
        "actions.create_event.event_template",
        event_title = markdown::escape(&base_event.title),
        event_description = markdown::escape(&base_event.description),
        event_subject = markdown::escape(&t!(&base_event.subject.to_string())),
        event_datetime = markdown::escape(&formatted_data),
        event_location = {
            format!(
                "📍 Место: _{}_",
                markdown::escape(&base_event.location_title)
            )
        },
        event_contact_info = match base_event.contact_info.as_deref() {
            None => "".to_string(),
            Some(contact_info) => format!("Контакт: _{}_", markdown::escape(contact_info)),
        },
    );

    match base_event.picture {
        Some(picture_uuid) => {
            let event_image_input_file =
                InputFile::file(get_event_image_path_by_uuid(picture_uuid));
            let mut msg = bot.send_photo(chat_id, event_image_input_file);
            msg.caption = Some(msg_text);
            msg.parse_mode = Some(ParseMode::MarkdownV2);
            // msg.reply_markup = Some(ReplyMarkup::InlineKeyboard(get_inline_kb_event_message(
            //     Some(base_event.location.get_yandex_map_link_to()),
            // )));
            msg.reply_markup = event_reply_markup;

            EventPostMessageRequest::WithPoster(msg)
        }
        None => {
            let mut msg = bot.send_message(chat_id, msg_text);
            msg.parse_mode = Some(ParseMode::MarkdownV2);
            msg.reply_markup = event_reply_markup;

            EventPostMessageRequest::Text(msg)
        }
    }
}
