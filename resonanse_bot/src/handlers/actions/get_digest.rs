use log::{debug, info};
use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::InputFile;
use teloxide::types::ParseMode::MarkdownV2;
use teloxide::utils::markdown;
use uuid::Uuid;

use resonanse_common::file_storage::get_feedback_images_path;
use resonanse_common::RecServiceClient;

use crate::{ACCOUNTS_REPOSITORY, EVENTS_REPOSITORY, MANAGER_BOT, REC_SERVICE_CLIENT};
use crate::config::{FEEDBACK_CHANNEL_ID, RABBITMQ_HOST};
use crate::handlers::{HandlerResult, MyDialogue};
use crate::handlers::utils::download_file_by_id;
use crate::utils::{prepare_event_list_view_with_marks, recommendation_subsystem_to_mark, repr_user_as_str};

pub async fn handle_get_digest_command(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    let accounts_repo = ACCOUNTS_REPOSITORY
        .get()
        .ok_or("Cannot get accounts repository")?;
    let events_repo = EVENTS_REPOSITORY
        .get()
        .ok_or("Cannot get events repository")?;

    // 1. send rpc request
    let rec_service_client = REC_SERVICE_CLIENT.get()
        .ok_or("Cannot get rpc service client")?;
    let user_id = accounts_repo.get_account_id_by_tg_user_id(
        msg.chat.id.0,
    ).await?;

    // todo  get user_id by tg_user_id
    let recommendation = rec_service_client
        .rpc_get_recommendation_by_user(user_id)
        .await?;

    debug!("recommendation {:?}", recommendation);

    // 2. prepare text view
    let events_ids: Vec<Uuid> = recommendation.iter().map(|rec_item| {
        rec_item.event_id
    }).collect();
    let events = events_repo.get_events_by_ids(events_ids).await?;

    let marks = recommendation.iter().map(|rec_item| {
        recommendation_subsystem_to_mark(&rec_item.subsystem)
    }).collect();
    let recommendation_view = prepare_event_list_view_with_marks(
        events,
        marks,
    );

    // 3. send to user
    let mut message = bot.send_message(
        msg.chat.id,
        recommendation_view,
    );
    message.parse_mode = Some(MarkdownV2);
    message.await?;
    Ok(())
}
