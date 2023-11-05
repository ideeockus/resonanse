use std::env;
use std::error::Error;

use log::debug;
use teloxide::payloads::{SendMessage, SendPhoto};
use teloxide::prelude::*;
use teloxide::requests::{JsonRequest, MultipartRequest};
use teloxide::types::{InputFile, ParseMode, ReplyMarkup};
use teloxide::utils::markdown;
use uuid::Uuid;

use resonanse_common::file_storage::get_event_image_path_by_uuid;
use resonanse_common::models::BaseEvent;
use resonanse_common::repository::CreateBaseEvent;

use crate::config::{DEFAULT_DATETIME_FORMAT, POSTS_CHANNEL_ID};
use crate::data_translators::fill_base_account_from_teloxide_user;
use crate::keyboards::get_inline_kb_event_message;
use crate::{ACCOUNTS_REPOSITORY, EVENTS_REPOSITORY, MANAGER_BOT};

pub async fn publish_event<I>(
    new_event: I,
    creator_tg_user: &teloxide::types::User,
) -> Result<BaseEvent, Box<dyn Error + Send + Sync>>
where
    CreateBaseEvent: From<I>,
{
    // save to db
    let user_account = fill_base_account_from_teloxide_user(creator_tg_user);
    let account = ACCOUNTS_REPOSITORY
        .get()
        .ok_or("Cannot get accounts repository")?
        .create_user_by_tg_user_id(user_account)
        .await?;

    let mut create_base_event = CreateBaseEvent::from(new_event);
    create_base_event.creator_id = account.id;
    let created_event = EVENTS_REPOSITORY
        .get()
        .ok_or("Cannot get events repository")?
        .create_event(create_base_event.clone())
        .await?;

    // post to tg
    if let Ok(tg_channel_to_post) = env::var(POSTS_CHANNEL_ID) {
        // if let Ok(tg_channel_to_post) = tg_channel_to_post.parse::<i64>() {
        debug!(
            "posting event {:?} to channel {}",
            created_event.id, tg_channel_to_post
        );
        let manager_bot = MANAGER_BOT.get().ok_or("Cannot get manager bot")?;

        // let mut message = manager_bot.send_message(
        //     tg_channel_to_post,
        //     create_base_event.format(),
        // );
        // message.reply_markup = Some(ReplyMarkup::InlineKeyboard(get_inline_kb_event_message(
        //     Some(created_event.location.get_yandex_map_link_to())
        // )));

        if let Ok(tg_channel_to_post) = tg_channel_to_post.parse::<i64>() {
            match prepare_event_msg_with_base_event(
                manager_bot,
                ChatId(tg_channel_to_post),
                created_event.clone(),
            ) {
                EventPostMessageRequest::WithPoster(f) => f.await?,
                EventPostMessageRequest::Text(f) => f.await?,
            };
        }
    }

    debug!("created event {:?}", created_event);
    Ok(created_event)
}

pub async fn send_event_post(
    bot: &Bot,
    chat_id: ChatId,
    event_uuid: Uuid,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let created_event = EVENTS_REPOSITORY
        .get()
        .ok_or("Cannot get events repository")?
        .get_event_by_uuid(event_uuid)
        .await?;

    match prepare_event_msg_with_base_event(bot, chat_id, created_event) {
        EventPostMessageRequest::WithPoster(f) => f.await?,
        EventPostMessageRequest::Text(f) => f.await?,
    };

    Ok(())
}

enum EventPostMessageRequest {
    WithPoster(MultipartRequest<SendPhoto>),
    Text(JsonRequest<SendMessage>),
}

fn prepare_event_msg_with_base_event(
    bot: &Bot,
    chat_id: ChatId,
    base_event: BaseEvent,
) -> EventPostMessageRequest {
    let msg_text = format!(
        r#"
*{}*
{}

💡 Тематика: _{}_
📅 Дата: _{}_
{}
{}
"#,
        markdown::escape(&base_event.title),
        markdown::escape(&base_event.description),
        markdown::escape(&base_event.subject.to_string()),
        markdown::escape(
            &base_event
                .datetime
                .format(DEFAULT_DATETIME_FORMAT)
                .to_string()
        ),
        match base_event.location.title.as_deref() {
            None => "".to_string(),
            Some(location_title) => format!("📍 Место: _{}_", markdown::escape(location_title)),
        },
        match base_event.contact_info.as_deref() {
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
            msg.reply_markup = Some(ReplyMarkup::InlineKeyboard(get_inline_kb_event_message(
                Some(base_event.location.get_yandex_map_link_to()),
            )));

            EventPostMessageRequest::WithPoster(msg)
        }
        None => {
            let msg = bot.send_message(chat_id, msg_text);

            EventPostMessageRequest::Text(msg)
        }
    }
}
