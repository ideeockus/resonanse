use std::env;

use uuid::Uuid;

use crate::config::RESONANSE_BOT_USERNAME;

// const TG_DOWNLOADS_PATH: &str = "tg_downloads";

// pub fn get_tg_downloads_dir() -> PathBuf {
//     let tg_downloads_path = Path::new(TG_DOWNLOADS_PATH);
//     if !tg_downloads_path.exists() {
//         fs::create_dir(tg_downloads_path).expect("Oops, cannot create dir");
//     }
//
//     tg_downloads_path.to_path_buf()
// }

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
