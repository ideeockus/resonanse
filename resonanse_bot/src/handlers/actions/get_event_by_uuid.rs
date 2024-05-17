use teloxide::prelude::{Message, Requester};
use teloxide::Bot;
use uuid::Uuid;

use crate::handlers::{HandlerResult, MyDialogue};
use crate::high_logics::send_event_post;
use crate::EVENTS_REPOSITORY;

pub async fn handle_get_event_by_uuid(
    bot: Bot,
    _dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    let events_repo = EVENTS_REPOSITORY
        .get()
        .ok_or("Cannot get events repository")?;

    // handle event command start
    if let Some(msg_text) = msg.text() {
        if let Some(rest_msg) = msg_text.strip_prefix("/event_") {
            if let Some(event_uuid) = rest_msg.split(' ').next() {
                if let Ok(event_uuid) = Uuid::parse_str(event_uuid) {
                    let choosed_event = events_repo.get_event_by_uuid(event_uuid).await;
                    if let Ok(choosed_event) = choosed_event {
                        send_event_post(&bot, msg.chat.id, choosed_event.id).await?;
                        return Ok(());
                    }
                }
            }
        }
    }

    // handle event command end
    bot.send_message(msg.chat.id, "Выбранное событие не найдено")
        .await?;

    Ok(())
}
