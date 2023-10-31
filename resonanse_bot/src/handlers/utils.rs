use crate::handlers::HandlerResult;
use crate::utils::get_tg_downloads_dir;
use std::path::Path;
use teloxide::net::Download;
use teloxide::prelude::*;
use teloxide::Bot;

pub async fn download_file_by_id(bot: &Bot, file_id: &str, dest_path: &Path) -> HandlerResult {
    let tg_file = bot.get_file(file_id).await?;

    // let mut async_fd = tokio::fs::File::create(get_downloads_dir().join(file_id)).await?;
    let mut async_fd = tokio::fs::File::create(dest_path).await?;
    bot.download_file(&tg_file.path, &mut async_fd).await?;

    Ok(())
}
