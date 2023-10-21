mod dispatch;
mod handlers;
mod keyboards;
mod user_settings;
mod commands;
mod states;
mod actions;

use dispatch::schema;
use env_logger;
use env_logger::{Builder, TimestampPrecision};
use log::{info, LevelFilter};
use teloxide::dptree;
use teloxide::prelude::*;
use user_settings::UserSettings;

#[tokio::main]
async fn main() {
    Builder::new()
        .filter_level(LevelFilter::Trace)
        .format_timestamp(Some(TimestampPrecision::Nanos))
        .init();

    run_polling()
}

pub async fn run_polling() {
    info!("Run telegram polling...");

    let bot = Bot::from_env();

    let update_handler = schema();
    let mut dispatcher = Dispatcher::builder(bot, update_handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build();

    dispatcher.dispatch().await;

    info!("Dispatcher started");
}
