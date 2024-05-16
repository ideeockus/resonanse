use std::env;
use std::sync::Arc;

use futures::future::BoxFuture;
use futures::FutureExt;
use teloxide::error_handlers::ErrorHandler;
use teloxide::prelude::*;
use teloxide::types::CallbackQuery;

use crate::config::ERROR_CHANNEL_ID;
use crate::handlers::HandlerResult;
use crate::MANAGER_BOT;

pub async fn log_msg_handler(msg: Message) -> HandlerResult {
    match msg.from() {
        None => {
            log::debug!("message from unknown user: {:?}", msg);
        }
        Some(user) => {
            log::debug!(
                "message from user {:?} [{}] - {}. {:?}",
                user.mention(),
                user.id,
                user.full_name(),
                msg,
            );
        }
    }

    Ok(())
}

pub async fn log_callback_handler(q: CallbackQuery) -> HandlerResult {
    let user = q.from.clone();
    log::debug!(
        "message from user {:?} [{}] - {}. {:?}",
        user.mention(),
        user.id,
        user.full_name(),
        q,
    );
    Ok(())
}

pub struct MyErrorHandler;

impl ErrorHandler<Box<dyn std::error::Error + Send + Sync>> for MyErrorHandler {
    fn handle_error(
        self: Arc<Self>,
        error: Box<dyn std::error::Error + Send + Sync>,
    ) -> BoxFuture<'static, ()> {
        async move {
            log::error!("Error handling update: {}", error);

            let manager_bot = if let Some(bot) = MANAGER_BOT.get() {
                bot
            } else {
                log::error!("Cannot get manager bot");
                return;
            };

            if let Ok(tg_feedback_chan) = env::var(ERROR_CHANNEL_ID) {
                if let Ok(tg_feedback_chan) = tg_feedback_chan.parse::<i64>() {
                    let tg_feedback_chan = ChatId(tg_feedback_chan);

                    let msg = format!("Error handling update: {}", error);
                    if let Err(err) = manager_bot.send_message(tg_feedback_chan, msg).send().await {
                        log::error!("Error sending message to feedback channel: {}", err);
                    }
                }
            }
        }
        .boxed()
    }
}
