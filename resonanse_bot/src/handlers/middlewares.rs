use crate::handlers::HandlerResult;
use teloxide::prelude::Message;
use teloxide::types::CallbackQuery;

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
