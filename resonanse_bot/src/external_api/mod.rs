use std::error::Error;
use std::path::{Path, PathBuf};

use resonanse_common::file_storage::get_event_image_path_by_uuid;
use resonanse_common::models::BaseEvent;
use uuid::Uuid;

use futures::StreamExt;
use tokio::fs::OpenOptions;

async fn download_image(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)
        .await?;
    let mut bytes_stream = response.bytes_stream();

    while let Some(chunk) = bytes_stream.next().await {
        let chunk = chunk?;
        tokio::io::copy(&mut chunk.as_ref(), &mut file).await?;
    }

    Ok(())
}

pub async fn resolve_event_picture(event: &mut BaseEvent) -> Option<PathBuf> {
    if let Some(local_path) = event.local_image_path.as_ref() {
        let local_path = get_event_image_path_by_uuid(local_path);
        if local_path.exists() {
            return Some(local_path);
        }
    }

    if let Some(image_url) = event.image_url.as_deref() {
        let picture_uuid = Uuid::new_v4();
        let local_path = get_event_image_path_by_uuid(picture_uuid);
        if download_image(image_url, &local_path).await.is_ok() {
            event.local_image_path = Some(picture_uuid.to_string());
            return Some(local_path);
        }
    }

    None
}
