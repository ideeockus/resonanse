use teloxide::Bot;
use teloxide::net::Download;
use teloxide::prelude::*;
use crate::handlers::HandlerResult;
use crate::utils::get_downloads_dir;

pub async fn download_file_by_id(bot: &Bot, file_id: &str) -> HandlerResult {
    let tg_file = bot.get_file(file_id).await?;

    let mut async_fd = tokio::fs::File::create(get_downloads_dir().join(file_id)).await?;
    bot.download_file(&tg_file.path, &mut async_fd).await?;

    Ok(())
}