use teloxide::payloads::{SendMessage, SendPhoto};
use teloxide::prelude::*;
use teloxide::requests::{JsonRequest, MultipartRequest};
use teloxide::types::{ChatId, InputFile, ParseMode, ReplyMarkup};
use teloxide::utils::markdown;
use teloxide::Bot;

use resonanse_common::file_storage::get_event_image_path_by_uuid;
use resonanse_common::models::BaseEvent;

use crate::config::DEFAULT_DATETIME_FORMAT;
use crate::external_api::resolve_event_picture;

pub enum EventPostMessageRequest {
    WithPoster(MultipartRequest<SendPhoto>),
    Text(JsonRequest<SendMessage>),
}

pub async fn prepare_event_msg_with_base_event(
    bot: &Bot,
    chat_id: ChatId,
    mut base_event: BaseEvent,
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
                base_event.datetime_from.format(DEFAULT_DATETIME_FORMAT),
                datetime_to.format(DEFAULT_DATETIME_FORMAT)
            )
        }
    };

    let stripped_description = base_event.get_description_up_to(700);

    let msg_text = t!(
        "actions.create_event.event_template",
        event_title = markdown::escape(&base_event.title),
        event_description = markdown::escape(&stripped_description),
        event_datetime = markdown::escape(&formatted_data),
        event_location = {
            format!(
                "📍 Место: _{}_",
                markdown::escape(&base_event.venue.get_name())
            )
        },
        event_contact_info = match base_event.contact.as_deref() {
            None => "".to_string(),
            Some(contact_info) => format!("Контакт: _{}_", markdown::escape(contact_info)),
        },
    );

    let event_picture_local_path = resolve_event_picture(&mut base_event).await;

    match event_picture_local_path {
        Some(picture_local_path) => {
            let event_image_input_file =
                InputFile::file(picture_local_path);
            let mut msg = bot.send_photo(chat_id, event_image_input_file);
            msg.caption = Some(msg_text);
            msg.parse_mode = Some(ParseMode::MarkdownV2);
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
