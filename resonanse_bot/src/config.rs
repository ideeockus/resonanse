use std::env;

pub const RESONANSE_BOT_TOKEN: &str = "RESONANSE_BOT_TOKEN";
pub const RESONANSE_MANAGEMENT_BOT_TOKEN: &str = "RESONANSE_MANAGEMENT_BOT_TOKEN";
pub const RESONANSE_BOT_USERNAME: &str = "RESONANSE_BOT_USERNAME";

// to limit user ability to publish infinity events
#[allow(unused)]
pub const RESONANSE_EVENT_PUBLICATION_LIMIT: &str = "RESONANSE_EVENT_PUBLICATION_LIMIT";
#[allow(unused)]
pub const RESONANSE_EVENT_PUBLICATION_LIMIT_RANGE: &str = "RESONANSE_EVENT_PUBLICATION_LIMIT_RANGE";
pub const FEEDBACK_CHANNEL_ID: &str = "FEEDBACK_CHANNEL_ID";
pub const ERROR_CHANNEL_ID: &str = "ERROR_CHANNEL_ID";
pub const POSTS_CHANNEL_ID: &str = "POSTS_CHANNEL_ID";

// user ids that able to manage service
pub const MANAGER_TG_IDS: &str = "MANAGER_TG_IDS";
pub const POSTGRES_HOST: &str = "POSTGRES_HOST";
pub const POSTGRES_PORT: &str = "POSTGRES_PORT";
pub const POSTGRES_DB: &str = "POSTGRES_DB";
pub const POSTGRES_PASSWORD: &str = "POSTGRES_PASSWORD";
pub const POSTGRES_USER: &str = "POSTGRES_USER";
pub const RABBITMQ_HOST: &str = "RABBITMQ_HOST";

pub const CLICKHOUSE_HOST: &str = "CLICKHOUSE_HOST";
pub const _CLICKHOUSE_USERNAME: &str = "CLICKHOUSE_USERNAME";
pub const _CLICKHOUSE_PASSWORD: &str = "CLICKHOUSE_PASSWORD";
pub const _CLICKHOUSE_DATABASE: &str = "CLICKHOUSE_DATABASE";

pub const DONATION_URL: &str = "DONATION_URL";
pub const WEB_APP_URL: &str = "WEB_APP_URL";

pub fn check_all_mandatory_envs_is_ok() {
    env::var(RESONANSE_BOT_TOKEN).unwrap();
    env::var(RESONANSE_MANAGEMENT_BOT_TOKEN).unwrap();
    env::var(RESONANSE_BOT_USERNAME).unwrap();
    env::var(FEEDBACK_CHANNEL_ID).unwrap();
    env::var(ERROR_CHANNEL_ID).unwrap();
    env::var(POSTS_CHANNEL_ID).unwrap();

    env::var(POSTGRES_USER).unwrap();
    env::var(POSTGRES_PASSWORD).unwrap();
    env::var(POSTGRES_DB).unwrap();

    env::var(RABBITMQ_HOST).unwrap();

    env::var(CLICKHOUSE_HOST).unwrap();

    env::var(MANAGER_TG_IDS).unwrap();

    env::var(WEB_APP_URL).unwrap();

    get_postgres_db_url();
}

pub fn get_postgres_db_url() -> String {
    let pg_host = env::var(POSTGRES_HOST).unwrap_or("localhost".to_string());
    let pg_port = env::var(POSTGRES_PORT).unwrap_or("5432".to_string());
    let pg_user = env::var(POSTGRES_USER).unwrap();
    let pg_password = env::var(POSTGRES_PASSWORD).unwrap();
    let pg_db = env::var(POSTGRES_DB).unwrap();

    format!(
        "postgresql://{}:{}@{}:{}/{}",
        pg_user, pg_password, pg_host, pg_port, pg_db,
    )
}

pub fn get_clickhouse_url() -> String {
    let clickhouse_host = env::var(CLICKHOUSE_HOST).unwrap_or("localhost".to_string());

    format!("http://{}:8123", clickhouse_host,)
}

// other
pub const DEFAULT_DATETIME_FORMAT: &str = "%d.%m.%Y %H:%M";
