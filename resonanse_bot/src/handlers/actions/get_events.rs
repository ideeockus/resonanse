use teloxide::Bot;
use teloxide::prelude::*;
use teloxide::types::Message;
use crate::EVENTS_REPOSITORY;
use crate::handlers::{HandlerResult, MyDialogue};
use crate::high_logics::send_event_post;

pub async fn handle_get_events(bot: Bot, dialogue: MyDialogue, (page_size, page_num): (i64, i64), msg: Message) -> HandlerResult {
    // handle event command start
    if let Some(msg_text) = msg.text() {
        if let Some(rest_msg) = msg_text.strip_prefix("/event_") {
            if let Some(event_num) = rest_msg.splitn(1, " ").next() {
                if let Ok(event_num) = event_num.parse::<i64>() {
                    // let event_global_num = event_num;
                    let events = EVENTS_REPOSITORY.get()
                        .ok_or("Cannot get events repository")?
                        .get_public_events(page_num, page_size)
                        .await?;

                    if let Some(choosed_event) = events.get(event_num as usize) {
                        send_event_post(&bot, msg.chat.id, choosed_event.id).await?;
                        return Ok(())
                    }
                }
            }
        }
    }
    // handle event command end

    bot.send_message(
        msg.chat.id,
        "Выбранное событие не найдено"
    ).await?;

    Ok(())
}