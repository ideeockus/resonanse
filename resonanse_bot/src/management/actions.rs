use std::env;
use std::str::FromStr;

use log::debug;
use teloxide::prelude::*;
use teloxide::types::{Message, ParseMode};
use teloxide::utils::command::parse_command;
use teloxide::utils::markdown;
use teloxide::Bot;
use uuid::Uuid;

use crate::config::MANAGER_TG_IDS;
use crate::management::common::HandlerResult;
use crate::{ACCOUNTS_REPOSITORY, EVENTS_REPOSITORY};

fn get_managers_ids() -> Vec<i64> {
    let managers_ids_str = env::var(MANAGER_TG_IDS).unwrap_or("".to_string());
    debug!("managers_ids_str: {:?}", managers_ids_str);
    let managers_ids = managers_ids_str
        .split(',')
        .filter_map(|mng_id_str| mng_id_str.parse::<i64>().ok())
        .collect();

    debug!("managers_ids: {:?}", managers_ids);
    managers_ids
}

pub async fn delete_event_command(bot: Bot, msg: Message) -> HandlerResult {
    debug!("got delete_event_command {:?}", &msg);

    // CHECK FOR MANAGER RIGHTS
    if !get_managers_ids().contains(&msg.chat.id.0) {
        return Ok(());
    }

    if let Some(command_text) = msg.text() {
        if let Some((_command, params)) = parse_command(command_text, "") {
            if let Some(first_param) = params.first() {
                if let Ok(event_uuid) = Uuid::from_str(first_param) {
                    let result = EVENTS_REPOSITORY
                        .get()
                        .ok_or("Cannot get events repository")?
                        .delete_event(event_uuid, msg.chat.id.0)
                        .await;

                    match result {
                        Ok(_) => {
                            bot.send_message(
                                msg.chat.id,
                                format!("Событие {} удалено", event_uuid),
                            )
                            .await?;
                        }
                        Err(_) => {
                            bot.send_message(
                                msg.chat.id,
                                format!("Событие {} НЕ удалено", event_uuid),
                            )
                            .await?;
                        }
                    }

                    return Ok(());
                }
            }
        }
    }

    let mut message = bot.send_message(msg.chat.id, "Команда не распознана");
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}

pub async fn get_stats_command(bot: Bot, msg: Message) -> HandlerResult {
    debug!("got get_stats_command {:?}", &msg);

    let count_accounts = ACCOUNTS_REPOSITORY
        .get()
        .ok_or("Cannot get accounts repository")?
        .count_accounts()
        .await?;

    // todo
    let _count_events_by_subject = EVENTS_REPOSITORY
        .get()
        .ok_or("Cannot get events repository")?
        .count_events_by_subject()
        .await?;

    // let all_events = events_uuids_map
    //     .iter()
    //     .map(|be| {
    //         format!(
    //             "*{}* \\- `{}`",
    //             markdown::escape(&be.title),
    //             markdown::escape(&be.id.to_string())
    //         )
    //     })
    //     .collect::<Vec<String>>()
    //     .join("\n");

    let mut message = bot.send_message(
        msg.chat.id,
        format!(
            "\\[пока статистика только такая\\]\nКоличество пользователей: {}\n\n",
            count_accounts,
        ),
    );

    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}

pub async fn search_event_command(
    bot: Bot,
    msg: Message,
    searching_event_title: String,
) -> HandlerResult {
    debug!("got search_event_command {:?}", &msg);

    // CHECK FOR MANAGER RIGHTS
    if !get_managers_ids().contains(&msg.chat.id.0) {
        return Ok(());
    }

    debug!("{:?}", msg.entities());

    debug!("search by substr {:?}", searching_event_title);

    let result = EVENTS_REPOSITORY
        .get()
        .ok_or("Cannot get events repository")?
        .get_events_by_title_substr(&searching_event_title)
        .await?;

    debug!("reesult {:?}", result);
    if !result.is_empty() {
        let result_formatted = result
            .iter()
            .map(|be| {
                format!(
                    "*{}* \\- `{}`",
                    markdown::escape(&be.title),
                    markdown::escape(&be.id.to_string())
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let mut message = bot.send_message(msg.chat.id, format!("События:\n{}", result_formatted));

        message.parse_mode = Some(ParseMode::MarkdownV2);
        message.await?;
        return Ok(());
    }

    let mut message = bot.send_message(msg.chat.id, "Ничего не найдено");
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}
