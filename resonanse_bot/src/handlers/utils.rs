use std::path::Path;

use teloxide::net::Download;
use teloxide::prelude::*;
use teloxide::Bot;
use uuid::Uuid;

use crate::data_structs::MyComplexCommand;
use crate::handlers::HandlerResult;

pub async fn download_file_by_id(bot: &Bot, file_id: &str, dest_path: &Path) -> HandlerResult {
    let tg_file = bot.get_file(file_id).await?;

    // let mut async_fd = tokio::fs::File::create(get_downloads_dir().join(file_id)).await?;
    let mut async_fd = tokio::fs::File::create(dest_path).await?;
    bot.download_file(&tg_file.path, &mut async_fd).await?;

    Ok(())
}

pub fn try_extract_event_id_from_text(text: &str) -> Option<MyComplexCommand> {
    if let Some(rest_text) = text.strip_prefix("/event_") {
        if let Some(event_str_id) = rest_text.split(' ').next() {
            if let Ok(event_uuid) = Uuid::parse_str(event_str_id) {
                return Some(MyComplexCommand::GetEventUuid(event_uuid));
            } else if let Ok(event_num_id) = event_str_id.parse::<i64>() {
                return Some(MyComplexCommand::GetEventIntId(event_num_id));
            }
        }
    }

    None
}
