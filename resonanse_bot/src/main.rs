#[macro_use]
extern crate rust_i18n;

use std::sync::{Arc, OnceLock};

use env_logger::{Builder, TimestampPrecision};
use log::{info, LevelFilter};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree;
use teloxide::prelude::*;
use tokio::sync::Mutex;

use dispatch::schema;
use resonanse_common::repository::{
    AccountsRepository, EventInteractionRepository, EventsRepository,
};
use resonanse_common::RecServiceClient;

use crate::config::{
    check_all_mandatory_envs_is_ok, CLICKHOUSE_DATABASE, CLICKHOUSE_DB_URL, CLICKHOUSE_PASSWORD,
    CLICKHOUSE_USERNAME, POSTGRES_DB_URL, RABBITMQ_HOST, RESONANSE_BOT_TOKEN,
};
use crate::handlers::MyErrorHandler;
use crate::management::run_resonanse_management_bot_polling;
use crate::states::BaseState;

mod commands;
mod config;
mod data_structs;
mod data_translators;
mod dispatch;
mod errors;
mod external_api;
mod handlers;
mod high_logics;
mod keyboards;
mod management;
mod states;
mod utils;

i18n!("locales", fallback = "ru");

static MANAGER_BOT: OnceLock<Bot> = OnceLock::new();
// static DB_POOL: OnceCell<resonanse_common::PgPool> = OnceCell::new();
static EVENTS_REPOSITORY: OnceLock<EventsRepository> = OnceLock::new();
static ACCOUNTS_REPOSITORY: OnceLock<AccountsRepository> = OnceLock::new();
static EVENTS_INTERACTION_REPOSITORY: OnceLock<EventInteractionRepository> = OnceLock::new();

static REC_SERVICE_CLIENT: OnceLock<RecServiceClient> = OnceLock::new();

#[tokio::main]
async fn main() {
    Builder::new()
        .filter(Some("hyper"), LevelFilter::Info)
        .filter(Some("reqwest"), LevelFilter::Info)
        .filter_level(LevelFilter::Debug)
        .format_timestamp(Some(TimestampPrecision::Nanos))
        .init();

    check_all_mandatory_envs_is_ok();
    setup_i18n_locales();

    let conn_url = std::env::var(POSTGRES_DB_URL).unwrap();
    let pool = resonanse_common::PgPool::connect(&conn_url).await.unwrap();
    let clickhouse_client = clickhouse::Client::default().with_url(CLICKHOUSE_DB_URL);
    // .with_user(CLICKHOUSE_USERNAME)
    // .with_password(CLICKHOUSE_PASSWORD)
    // .with_database(CLICKHOUSE_DATABASE);

    // initialize repositories
    let events_repository = EventsRepository::new(pool.clone());
    EVENTS_REPOSITORY.set(events_repository).unwrap();

    let accounts_repository = AccountsRepository::new(pool.clone());
    ACCOUNTS_REPOSITORY.set(accounts_repository).unwrap();

    let events_scores_repository = EventInteractionRepository::new(pool.clone(), clickhouse_client);
    EVENTS_INTERACTION_REPOSITORY
        .set(events_scores_repository)
        .unwrap();

    // initialize RPC client
    let rabbitmq_host = std::env::var(RABBITMQ_HOST).unwrap();
    REC_SERVICE_CLIENT
        .set(RecServiceClient::new(&rabbitmq_host).await)
        .unwrap();

    let resonanse_bot_handle = tokio::spawn(async { run_resonanse_bot_polling().await });
    let _resonanse_management_bot_handle =
        tokio::spawn(async { run_resonanse_management_bot_polling().await });

    resonanse_bot_handle.await.unwrap()
}

pub async fn run_resonanse_bot_polling() {
    info!("Run telegram resonanse bot polling...");

    let resonanse_bot_token = std::env::var(RESONANSE_BOT_TOKEN).unwrap();
    let bot = Bot::new(resonanse_bot_token);

    let error_handler = Arc::new(MyErrorHandler);

    let update_handler = schema();
    let mut dispatcher = Dispatcher::builder(bot, update_handler)
        .dependencies(dptree::deps![InMemStorage::<BaseState>::new()])
        .error_handler(error_handler)
        .enable_ctrlc_handler()
        .build();

    dispatcher.dispatch().await;

    info!("Dispatcher started");
}

fn setup_i18n_locales() {
    rust_i18n::set_locale("ru");
    info!(
        "available rust_i18n locales: {:?}",
        rust_i18n::available_locales!()
    );
    info!("default rust_i18n locale: {:?}", rust_i18n::locale());
}

#[allow(unused)]
fn run_migrations() {
    info!("running sqlx migrate");
}
