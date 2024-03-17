use std::env;
use std::error::Error;

use log::{debug};

use teloxide::prelude::*;
use teloxide::types::ReplyMarkup;
use uuid::Uuid;

use resonanse_common::models::BaseEvent;
// use resonanse_common::repository::CreateBaseEvent;

use crate::config::POSTS_CHANNEL_ID;
use crate::data_structs::{prepare_event_msg_with_base_event, EventPostMessageRequest};
use crate::data_translators::fill_base_account_from_teloxide_user;
use crate::errors::BotHandlerError;
use crate::keyboards::get_inline_kb_event_message;
use crate::{ACCOUNTS_REPOSITORY, EVENTS_REPOSITORY, MANAGER_BOT};

pub async fn publish_event<I>(
    new_event: I,
    creator_tg_user: &teloxide::types::User,
) -> Result<BaseEvent, Box<dyn Error + Send + Sync>>
where
    BaseEvent: TryFrom<I>,
{
    // save to db
    let user_account = fill_base_account_from_teloxide_user(creator_tg_user);
    let account = ACCOUNTS_REPOSITORY
        .get()
        .ok_or("Cannot get accounts repository")?
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

    // todo: is this check necessary ?
    if create_base_event.picture.is_none() {
        return Err(Box::new(BotHandlerError::UnfilledEvent));
    }

    // make brief event description
    // todo move exter api calls to another module
    // THIS EXTERNAL API CALL NOW IS NOT USING
    // let client = reqwest::Client::new();
    // let instance_data = SberSummarizatorInstance::new(create_base_event.description.clone());
    // let req_json_data = HashMap::from([("instances", [instance_data])]);
    // match client
    //     .post("https://api.aicloud.sbercloud.ru/public/v2/summarizator/predict")
    //     .json(&req_json_data)
    //     .send()
    //     .await
    // {
    //     Ok(resp) => {
    //         let j = resp.json::<serde_json::Value>().await;
    //         debug!("j {:?}", j);
    //         if let Ok(v) = j {
    //             if let Some(prediction_best) = v
    //                 .get("prediction_best")
    //                 .and_then(|v| v.get("bertscore"))
    //                 .and_then(|v| v.as_str())
    //             {
    //                 create_base_event.brief_description = Some(prediction_best.to_string());
    //             }
    //         }
    //     }
    //     Err(err) => {
    //         warn!("cannot summarize description: {:?}", err);
    //     }
    // }

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
                construct_created_event_kb(&created_event),
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

    // let event_inline_btns = match created_event.location {
    //     None => None,
    //     Some(location) => Some(),
    // };
    let event_post_message_request = prepare_event_msg_with_base_event(
        bot,
        chat_id,
        created_event.clone(),
        construct_created_event_kb(&created_event),
    );
    match event_post_message_request {
        EventPostMessageRequest::WithPoster(f) => f.await?,
        EventPostMessageRequest::Text(f) => f.await?,
    };

    Ok(())
}

pub fn construct_created_event_kb(created_event: &BaseEvent) -> Option<ReplyMarkup> {
    Some(ReplyMarkup::InlineKeyboard(get_inline_kb_event_message(
        created_event.id,
        created_event
            .location
            .as_ref()
            .map(|loc| loc.get_yandex_map_link_to()),
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
