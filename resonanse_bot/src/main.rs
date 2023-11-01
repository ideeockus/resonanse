use env_logger;
use env_logger::{Builder, TimestampPrecision};
use log::{info, LevelFilter};

use std::sync::OnceLock;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree;
use teloxide::prelude::*;

use crate::config::{
    check_all_mandatory_envs_is_ok, POSTGRES_DB_URL, RESONANSE_BOT_TOKEN,
    RESONANSE_MANAGEMENT_BOT_TOKEN,
};
use crate::management::run_resonanse_management_bot_polling;
use dispatch::schema;
use resonanse_common::repository::{AccountsRepository, EventsRepository};

use crate::states::BaseState;

mod commands;
mod config;
mod data_translators;
mod dispatch;
mod errors;
mod handlers;
mod high_logics;
mod keyboards;
mod management;
mod states;
mod user_settings;
mod utils;

static MANAGER_BOT: OnceLock<Bot> = OnceLock::new();
// static DB_POOL: OnceCell<resonanse_common::PgPool> = OnceCell::new();
static EVENTS_REPOSITORY: OnceLock<EventsRepository> = OnceLock::new();
static ACCOUNTS_REPOSITORY: OnceLock<AccountsRepository> = OnceLock::new();

#[tokio::main]
async fn main() {
    Builder::new()
        .filter(Some("hyper"), LevelFilter::Info)
        .filter(Some("reqwest"), LevelFilter::Info)
        .filter_level(LevelFilter::Debug)
        .format_timestamp(Some(TimestampPrecision::Nanos))
        .init();

    check_all_mandatory_envs_is_ok();

    let conn_url = std::env::var(POSTGRES_DB_URL).unwrap();
    let pool = resonanse_common::PgPool::connect(&conn_url).await.unwrap();
    // DB_POOL.set(pool).unwrap();
    let events_repository = EventsRepository::new(pool.clone());
    EVENTS_REPOSITORY.set(events_repository).unwrap();

    let accounts_repository = AccountsRepository::new(pool.clone());
    ACCOUNTS_REPOSITORY.set(accounts_repository).unwrap();

    let resonanse_bot_handle = tokio::spawn(async { run_resonanse_bot_polling().await });
    let _resonanse_management_bot_handle =
        tokio::spawn(async { run_resonanse_management_bot_polling().await });

    resonanse_bot_handle.await.unwrap()
}

pub async fn run_resonanse_bot_polling() {
    info!("Run telegram resonanse bot polling...");

    let resonanse_bot_token = std::env::var(RESONANSE_BOT_TOKEN).unwrap();
    let bot = Bot::new(resonanse_bot_token);

    let update_handler = schema();
    let mut dispatcher = Dispatcher::builder(bot, update_handler)
        .dependencies(dptree::deps![InMemStorage::<BaseState>::new()])
        .enable_ctrlc_handler()
        .build();

    dispatcher.dispatch().await;

    info!("Dispatcher started");
}
