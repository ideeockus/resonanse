use std::env;

pub const RESONANSE_BOT_TOKEN: &str = "RESONANSE_BOT_TOKEN";
pub const RESONANSE_MANAGEMENT_BOT_TOKEN: &str = "RESONANSE_MANAGEMENT_BOT_TOKEN";
pub const RESONANSE_BOT_USERNAME: &str = "RESONANSE_BOT_USERNAME";

// to limit user ability to publish infinity events
pub const RESONANSE_EVENT_PUBLICATION_LIMIT: &str = "RESONANSE_EVENT_PUBLICATION_LIMIT";
pub const RESONANSE_EVENT_PUBLICATION_LIMIT_RANGE: &str = "RESONANSE_EVENT_PUBLICATION_LIMIT_RANGE";
pub const FEEDBACK_CHANNEL_ID: &str = "FEEDBACK_CHANNEL_ID";
pub const POSTS_CHANNEL_ID: &str = "POSTS_CHANNEL_ID";

// user ids that able to manage service
pub const MANAGER_TG_IDS: &str = "MANAGER_TG_IDS";
pub const POSTGRES_DB_URL: &str = "POSTGRES_DB_URL";

pub fn check_all_mandatory_envs_is_ok() {
    env::var(RESONANSE_BOT_TOKEN).unwrap();
    env::var(RESONANSE_MANAGEMENT_BOT_TOKEN).unwrap();
    env::var(RESONANSE_BOT_USERNAME).unwrap();
    env::var(FEEDBACK_CHANNEL_ID).unwrap();
    env::var(POSTS_CHANNEL_ID).unwrap();

    env::var(POSTGRES_DB_URL).unwrap();
}


// other
pub const DEFAULT_DATETIME_FORMAT: &str = "%d.%m.%Y %H:%M";