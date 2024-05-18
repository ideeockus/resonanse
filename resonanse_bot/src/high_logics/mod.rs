use std::env;
use std::error::Error;

use log::{debug, error};
use serde_json::json;

use teloxide::prelude::*;
use teloxide::types::ReplyMarkup;
use uuid::Uuid;

use resonanse_common::models::BaseEvent;

use crate::config::POSTS_CHANNEL_ID;
use crate::data_structs::{prepare_event_msg_with_base_event, EventPostMessageRequest};
use crate::data_translators::fill_base_account_from_teloxide_user;
use crate::errors::BotHandlerError;
use crate::keyboards::get_inline_kb_event_message;
use crate::{ACCOUNTS_REPOSITORY, EVENTS_INTERACTION_REPOSITORY, EVENTS_REPOSITORY, MANAGER_BOT};

pub async fn publish_event<I>(
    new_event: I,
    creator_tg_user: &teloxide::types::User,
) -> Result<BaseEvent, Box<dyn Error + Send + Sync>>
where
    BaseEvent: TryFrom<I>,
{
    let accounts_repo = ACCOUNTS_REPOSITORY
        .get()
        .ok_or("Cannot get accounts repository")?;
    let events_repo = EVENTS_REPOSITORY
        .get()
        .ok_or("Cannot get events repository")?;

    // save to db
    let user_account = fill_base_account_from_teloxide_user(creator_tg_user);
    let account = accounts_repo
        .create_user_by_tg_user_id(user_account)
        .await?;

    let mut create_base_event: BaseEvent = match new_event
        .try_into()
        .map_err(|_e| BotHandlerError::UnfilledEvent)
    {
        Ok(v) => v,
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    create_base_event.service_data = Some(json!({"creator_id": account.id}));
    let created_event = events_repo.create_event(create_base_event.clone()).await?;

    // post to tg
    if let Ok(tg_channel_to_post) = env::var(POSTS_CHANNEL_ID) {
        debug!(
            "posting event {:?} to channel {}",
            created_event.id, tg_channel_to_post
        );
        let manager_bot = MANAGER_BOT.get().ok_or("Cannot get manager bot")?;

        if let Ok(tg_channel_to_post) = tg_channel_to_post.parse::<i64>() {
            match prepare_event_msg_with_base_event(
                manager_bot,
                ChatId(tg_channel_to_post),
                created_event.clone(),
                construct_created_event_kb(&created_event),
            )
            .await
            {
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
    let events_repo = EVENTS_REPOSITORY
        .get()
        .ok_or("Cannot get events repository")?;
    let accounts_repo = ACCOUNTS_REPOSITORY
        .get()
        .ok_or("Cannot get accounts repository")?;
    let events_interaction_repo = EVENTS_INTERACTION_REPOSITORY
        .get()
        .ok_or("Cannot get events interaction repository")?;
    // save this user click/view of event to db
    let user_account_id = accounts_repo
        .get_account_id_by_tg_user_id(chat_id.0)
        .await?;
    if let Err(err) = events_interaction_repo
        .add_event_click(user_account_id, event_uuid)
        .await
    {
        error!("Cannot save event click action: {:?}", err)
    }

    // then send event post to this user
    let event = events_repo.get_event_by_uuid(event_uuid).await?;

    let event_post_message_request = prepare_event_msg_with_base_event(
        bot,
        chat_id,
        event.clone(),
        construct_created_event_kb(&event),
    )
    .await;
    match event_post_message_request {
        EventPostMessageRequest::WithPoster(f) => f.await?,
        EventPostMessageRequest::Text(f) => f.await?,
    };

    Ok(())
}

pub fn construct_created_event_kb(created_event: &BaseEvent) -> Option<ReplyMarkup> {
    Some(ReplyMarkup::InlineKeyboard(get_inline_kb_event_message(
        created_event.id,
        created_event.venue.get_yandex_map_link_to(),
    )))
}

#[derive(serde::Serialize)]
pub struct SberSummarizatorInstance {
    text: String,
    num_beams: i32,
    num_return_sequences: i32,
    length_penalty: f64,
}

impl SberSummarizatorInstance {
    #[allow(unused)]
    pub fn new(text: String) -> Self {
        Self {
            text,
            num_beams: 5,
            num_return_sequences: 3,
            length_penalty: 0.5,
        }
    }
}
