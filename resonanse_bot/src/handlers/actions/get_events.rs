use std::error::Error;

use log::debug;

use teloxide::prelude::*;
use teloxide::types::{Message, ParseMode, ReplyMarkup};
use teloxide::utils::markdown;
use teloxide::Bot;

use resonanse_common::models::EventSubject;
use resonanse_common::EventSubjectFilter;

use crate::handlers::{HandlerResult, MyDialogue};
use crate::high_logics::send_event_post;
use crate::keyboards::{get_inline_kb_events_page, get_inline_kb_set_subject_filter};
use crate::states::BaseState;
use crate::{keyboards, EVENTS_REPOSITORY};

pub async fn handle_get_events(
    bot: Bot,
    _dialogue: MyDialogue,
    (page_size, page_num, events_filter): (i64, i64, EventSubjectFilter),
    msg: Message,
) -> HandlerResult {
    // handle event command start
    if let Some(msg_text) = msg.text() {
        if let Some(rest_msg) = msg_text.strip_prefix("/event_") {
            if let Some(event_num) = rest_msg.split(' ').next() {
                if let Ok(event_num) = event_num.parse::<i64>() {
                    // let event_global_num = event_num;
                    let events = EVENTS_REPOSITORY
                        .get()
                        .ok_or("Cannot get events repository")?
                        .get_public_events(page_num, page_size, &events_filter)
                        .await?;

                    if let Some(choosed_event) = events.get(event_num as usize - 1) {
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

pub async fn handle_get_events_callback(
    bot: Bot,
    dialogue: MyDialogue,
    (page_size, page_num, events_filter): (i64, i64, EventSubjectFilter),
    q: CallbackQuery,
) -> HandlerResult {
    debug!("got handle_get_events__callback callback");
    bot.answer_callback_query(q.id.clone()).await?;

    match q.data.as_deref() {
        None => {
            bot.send_message(q.from.id, "Действие не распознано")
                .await?;
            Ok(())
        }
        Some(keyboards::EVENTS_PAGE_LEFT | keyboards::EVENTS_PAGE_RIGHT) => {
            handle_page_callback(bot, dialogue, (page_size, page_num, events_filter), q).await
        }
        _ => {
            handle_events_filter_callback(bot, dialogue, (page_size, page_num, events_filter), q)
                .await
        }
    }
}

pub async fn handle_events_filter_callback(
    bot: Bot,
    dialogue: MyDialogue,
    (page_size, page_num, mut events_filter): (i64, i64, EventSubjectFilter),
    q: CallbackQuery,
) -> HandlerResult {
    debug!("got handle_events_filter_callback callback");

    let msg = match q.message {
        None => {
            bot.send_message(q.from.id, "Unknown message").await?;
            return Ok(());
        }
        Some(v) => v,
    };

    match q.data.as_deref() {
        None => (),
        Some(keyboards::APPLY_EVENT_FILTER_BTN) => {
            bot.delete_message(msg.chat.id, msg.id).await?;

            let msg_text = get_choose_event_text(page_num, page_size, &events_filter).await?;
            let mut message = bot.send_message(q.from.id, msg_text);
            message.reply_markup = Some(ReplyMarkup::InlineKeyboard(get_inline_kb_events_page()));
            message.parse_mode = Some(ParseMode::MarkdownV2);
            message.await?;
            return Ok(());
        }
        Some(text) => match EventSubject::try_from(text) {
            Ok(event_subject) => {
                events_filter.switch(event_subject);
                let mut edit_msg = bot.edit_message_reply_markup(msg.chat.id, msg.id);
                edit_msg.reply_markup = Some(get_inline_kb_set_subject_filter(&events_filter));
                edit_msg.await?;
            }
            Err(_e) => {
                bot.send_message(q.from.id, "Не распознанное действие")
                    .await?;
            }
        },
    };

    dialogue
        .update(BaseState::GetEventList {
            page_size,
            page_num,
            events_filter,
        })
        .await?;

    Ok(())
}

pub async fn handle_page_callback(
    bot: Bot,
    dialogue: MyDialogue,
    (page_size, page_num, events_filter): (i64, i64, EventSubjectFilter),
    q: CallbackQuery,
) -> HandlerResult {
    debug!("got handle_page_callback callback");
    let page_num = page_num as u32;

    let msg = match q.message {
        None => {
            bot.send_message(q.from.id, "Unknown message").await?;
            return Ok(());
        }
        Some(v) => v,
    };

    let (page_size, page_num) = match q.data.as_deref() {
        Some(keyboards::EVENTS_PAGE_LEFT) => (page_size, page_num.saturating_sub(1)),
        Some(keyboards::EVENTS_PAGE_RIGHT) => (page_size, page_num + 1),
        _ => {
            return Ok(());
        }
    };

    let page_num = page_num as i64;

    dialogue
        .update(BaseState::GetEventList {
            page_size,
            page_num,
            events_filter: events_filter.clone(),
        })
        .await?;

    let msg_text = get_choose_event_text(page_num, page_size, &events_filter).await?;
    let mut message = bot.edit_message_text(msg.chat.id, msg.id, msg_text);
    message.reply_markup = Some(get_inline_kb_events_page());
    message.parse_mode = Some(ParseMode::MarkdownV2);

    message.await?;
    Ok(())
}

pub async fn get_choose_event_text(
    page_num: i64,
    page_size: i64,
    events_filter: &EventSubjectFilter,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let events = EVENTS_REPOSITORY
        .get()
        .ok_or("Cannot get events repository")?
        .get_public_events(page_num, page_size, events_filter)
        .await?;

    let mut event_i = 0;
    // let msg_text = format!(
    //     "_Страница {}_\nВыбери интересное событие и нажми на его идентификатор\n\n{}",
    //     markdown::escape(&page_num.to_string()),
    //     events
    //         .iter()
    //         .map(|event| {
    //             event_i += 1;
    //             format!(
    //                 "*{}*\t\t/event\\_{}",
    //                 markdown::escape(&event.title),
    //                 event_i
    //             )
    //         })
    //         .collect::<Vec<String>>()
    //         .join("\n\n")
    // );

    let msg_text = t!(
        "event_page.page_title",
        page_num = markdown::escape(&page_num.to_string()),
        page_data = events
            .iter()
            .map(|event| {
                event_i += 1;

                debug!("event.brief_description {:?}", event.brief_description);
                let event_brief_description_text = match event.brief_description.as_deref() {
                    Some(brief_desc) => format!("\n_{}_", markdown::escape(brief_desc),),
                    None => String::new(),
                };

                format!(
                    "/event\\_{}\t*{}*{}",
                    event_i,
                    markdown::escape(&event.title),
                    event_brief_description_text,
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    );

    Ok(msg_text)
}
