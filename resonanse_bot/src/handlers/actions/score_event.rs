use log::debug;
use teloxide::prelude::*;
use teloxide::Bot;
use uuid::Uuid;

use resonanse_common::models::EventScoreType;
use resonanse_common::repository::{AccountsRepository, EventScoresRepository};

use crate::config::POSTGRES_DB_URL;
use crate::handlers::HandlerResult;
use crate::keyboards;

pub fn score_event_handler(q: CallbackQuery) -> bool {
    let q_data = q.data.unwrap_or_default();
    q_data.starts_with(keyboards::INLINE_LIKE_EVENT_BTN)
        || q_data.starts_with(keyboards::INLINE_DISLIKE_EVENT_BTN)
}

pub async fn handle_score_event_callback(bot: Bot, q: CallbackQuery) -> HandlerResult {
    debug!("got handle_score_event_callback callback");

    bot.answer_callback_query(q.id).await?;
    let msg = match q.message {
        None => {
            bot.send_message(q.from.id, "Unknown message").await?;
            return Ok(());
        }
        Some(v) => v,
    };

    let conn_url = std::env::var(POSTGRES_DB_URL).unwrap();
    let pool = resonanse_common::PgPool::connect(&conn_url).await?;
    let events_score_repository = EventScoresRepository::new(pool.clone());
    let accounts_repository = AccountsRepository::new(pool.clone());

    let user_account = accounts_repository
        .get_user_by_tg_id(q.from.id.0 as i64)
        .await?;

    match q.data.as_deref() {
        None => (),
        Some(like_text) if like_text.starts_with(keyboards::INLINE_LIKE_EVENT_BTN) => {
            let event_id =
                Uuid::parse_str(&like_text.replace(keyboards::INLINE_LIKE_EVENT_BTN, ""))?;

            events_score_repository
                .set_event_score_by_user(event_id, user_account.id, EventScoreType::Like)
                .await?;

            debug!("inline kb {:?}", msg.reply_markup());
            if let Some(inline_kb) = msg.reply_markup() {
                let mut inline_kb = inline_kb.clone();
                if let Some(btn) = inline_kb
                    .inline_keyboard
                    .first_mut()
                    .and_then(|row| row.first_mut())
                {
                    btn.text = t!("keyboards.liked_event_btn");
                }
                if let Some(btn) = inline_kb
                    .inline_keyboard
                    .first_mut()
                    .and_then(|row| row.last_mut())
                {
                    btn.text = t!("keyboards.dislike_event_btn");
                }
                let mut edit_msg = bot.edit_message_reply_markup(msg.chat.id, msg.id);
                edit_msg.reply_markup = Some(inline_kb);
                edit_msg.await?;
            }

            return Ok(());
        }
        Some(like_text) if like_text.starts_with(keyboards::INLINE_DISLIKE_EVENT_BTN) => {
            let event_id =
                Uuid::parse_str(&like_text.replace(keyboards::INLINE_DISLIKE_EVENT_BTN, ""))?;

            events_score_repository
                .set_event_score_by_user(event_id, user_account.id, EventScoreType::Dislike)
                .await?;

            debug!("inline kb {:?}", msg.reply_markup());
            if let Some(inline_kb) = msg.reply_markup() {
                let mut inline_kb = inline_kb.clone();
                if let Some(btn) = inline_kb
                    .inline_keyboard
                    .first_mut()
                    .and_then(|row| row.first_mut())
                {
                    btn.text = t!("keyboards.like_event_btn");
                }
                if let Some(btn) = inline_kb
                    .inline_keyboard
                    .first_mut()
                    .and_then(|row| row.last_mut())
                {
                    btn.text = t!("keyboards.disliked_event_btn");
                }
                let mut edit_msg = bot.edit_message_reply_markup(msg.chat.id, msg.id);
                edit_msg.reply_markup = Some(inline_kb);
                edit_msg.await?;
            }

            return Ok(());
        }
        another_text => {
            debug!("Another score callback text: {:?}", another_text);
        }
    };

    Ok(())
}
