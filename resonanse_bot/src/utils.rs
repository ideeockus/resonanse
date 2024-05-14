use std::env;

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
