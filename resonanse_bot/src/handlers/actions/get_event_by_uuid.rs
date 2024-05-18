use teloxide::prelude::{Message, Requester};
use teloxide::Bot;

use crate::data_structs::MyComplexCommand;
use crate::handlers::{try_extract_event_id_from_text, HandlerResult, MyDialogue};
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
        if let Some(MyComplexCommand::GetEventUuid(event_uuid)) =
            try_extract_event_id_from_text(msg_text)
        {
            let choosed_event = events_repo.get_event_by_uuid(event_uuid).await;
            if let Ok(choosed_event) = choosed_event {
                send_event_post(&bot, msg.chat.id, choosed_event.id).await?;
                return Ok(());
            }
        }
    }

    // handle event command end
    bot.send_message(msg.chat.id, "Выбранное событие не найдено")
        .await?;

    Ok(())
}
