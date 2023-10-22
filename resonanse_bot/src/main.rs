use env_logger;
use env_logger::{Builder, TimestampPrecision};
use log::{info, LevelFilter};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dptree;
use teloxide::prelude::*;

use dispatch::schema;

use crate::states::BaseState;

mod dispatch;
mod handlers;
mod keyboards;
mod user_settings;
mod commands;
mod states;
mod actions;

#[tokio::main]
async fn main() {
    Builder::new()
        .filter_level(LevelFilter::Trace)
        .format_timestamp(Some(TimestampPrecision::Nanos))
        .init();

    run_polling().await;
}

pub async fn run_polling() {
    info!("Run telegram polling...");

    let bot = Bot::from_env();

    let update_handler = schema();
    let mut dispatcher = Dispatcher::builder(bot, update_handler)
        .dependencies(dptree::deps![InMemStorage::<BaseState>::new()])
        .enable_ctrlc_handler()
        .build();

    dispatcher.dispatch().await;

    info!("Dispatcher started");
}
