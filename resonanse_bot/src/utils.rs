use std::env;
use teloxide::utils::markdown;

use resonanse_common::models::{BaseEvent, RecSubsystem};
use uuid::Uuid;

use crate::config::RESONANSE_BOT_USERNAME;

#[inline]
pub fn repr_user_as_str(user: Option<&teloxide::types::User>) -> String {
    match user {
        None => "Unknown user".to_string(),
        Some(user) => {
            format!(
                "{} {} {} [{}]",
                user.first_name,
                user.last_name.as_ref().unwrap_or(&String::new()),
                user.username
                    .as_ref()
                    .map(|username| { format!("@{}", username) })
                    .unwrap_or(String::new()),
                user.id,
            )
        }
    }
}

pub fn build_event_deep_link(event_uuid: Uuid) -> String {
    let bot_username = env::var(RESONANSE_BOT_USERNAME);
    let bot_username = bot_username.as_deref().unwrap_or("resonanse_bot");

    build_deep_link_with_param(bot_username, &format!("event_{}", &event_uuid.to_string()))
}

pub fn build_deep_link_with_param(bot_username: &str, param: &str) -> String {
    format!("https://t.me/{}?start={}", bot_username, param)
}

pub fn prepare_event_list_view(events: Vec<BaseEvent>) -> String {
    let mut event_i = 0;
    events
        .iter()
        .map(|event| {
            event_i += 1;
            format!(
                "/event\\_{}\t{}\n⏰ {}\n📍 {}",
                event_i,
                markdown::escape(&event.title),
                markdown::escape(&event.datetime_from.to_string()),
                markdown::escape(&event.venue.get_name()),
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}

pub fn prepare_event_list_view_with_marks(events: Vec<BaseEvent>, marks: Vec<String>) -> String {
    let mut event_i = 0;
    events
        .iter()
        .map(|event| {
            let mark = marks[event_i].as_str();
            event_i += 1;

            format!(
                "{} /event\\_{}\t{}\n⏰ {}\n📍 {}",
                mark,
                event_i,
                markdown::escape(&event.title),
                markdown::escape(&event.datetime_from.to_string()),
                markdown::escape(&event.venue.get_name()),
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n")
}

pub fn recommendation_subsystem_to_mark(subsystem: &RecSubsystem) -> String {
    match subsystem {
        RecSubsystem::Basic => "🔵".to_string(),
        RecSubsystem::Dynamic => "🟠".to_string(),
        RecSubsystem::Collaborative => "🟣".to_string(),
    }
}
