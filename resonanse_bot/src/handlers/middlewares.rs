use teloxide::handlers::HandlerResult;
use teloxide::prelude::Message;

pub async fn log_request_handler(msg: Message) -> HandlerResult {
    match msg.from() {
        None => {
            log::debug!("message from unknown user");
        }
        Some(user) => {
            log::debug!(
                "message from user {:?} [{}] - {}. special: {}|{}",
                user.mention(),
                user.id,
                user.full_name(),
                user.is_anonymous(),
                user.is_telegram(),
            );
        }
    }

    Ok(())
}