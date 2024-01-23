use crate::config::RESONANSE_MANAGEMENT_BOT_TOKEN;
use crate::management::dispatch::manager_schema;
use crate::MANAGER_BOT;
use log::info;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dispatcher;
use teloxide::{dptree, Bot};

mod actions;
mod commands;
mod common;
mod dispatch;

#[derive(Clone, Default)]
pub enum BaseManagementState {
    #[default]
    Start,
    #[allow(unused)]
    Idle,
}

pub async fn run_resonanse_management_bot_polling() {
    info!("Run telegram resonanse management bot polling...");

    let resonanse_mngmnt_bot_token = std::env::var(RESONANSE_MANAGEMENT_BOT_TOKEN).unwrap();
    let manager_bot = Bot::new(resonanse_mngmnt_bot_token);
    MANAGER_BOT.set(manager_bot.clone()).unwrap();

    let update_handler = manager_schema();

    // todo change handlers
    let mut dispatcher = Dispatcher::builder(manager_bot, update_handler)
        .dependencies(dptree::deps![InMemStorage::<BaseManagementState>::new()])
        .enable_ctrlc_handler()
        .build();

    dispatcher.dispatch().await;

    info!("Dispatcher started");
}
