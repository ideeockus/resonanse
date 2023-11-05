use crate::handlers::HandlerResult;
use teloxide::prelude::Message;

pub async fn log_request_handler(msg: Message) -> HandlerResult {
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
