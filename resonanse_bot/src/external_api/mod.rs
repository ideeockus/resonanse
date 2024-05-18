use std::error::Error;
use std::path::{Path, PathBuf};

use futures::StreamExt;
use image::imageops::FilterType;
use image::io::Reader;
use image::{GenericImageView, ImageFormat};
use log::{debug, error};
use tokio::fs::OpenOptions;
use uuid::Uuid;

use crate::EVENTS_REPOSITORY;
use resonanse_common::file_storage::get_event_image_path_by_uuid;
use resonanse_common::models::BaseEvent;

pub fn resize_and_overwrite_image(
    file_path: &Path,
    max_size: u32,
) -> Result<(), image::ImageError> {
    // Open the image from the path
    let img = match Reader::open(file_path)?.with_guessed_format()?.decode() {
        Ok(v) => v,
        Err(err) => {
            error!("Cannot open image {:?}: {:?}", file_path, err);
            return Err(err);
        }
    };

    // Calculate the new dimensions while preserving the aspect ratio
    let (width, height) = img.dimensions();
    if width > max_size || height > max_size {
        debug!("resizing image {:?} from {:?}", file_path, (width, height));
        let aspect_ratio = width as f32 / height as f32;
        let (new_width, new_height) = if width > height {
            (max_size, (max_size as f32 / aspect_ratio) as u32)
        } else {
            ((max_size as f32 * aspect_ratio) as u32, max_size)
        };

        // Resize the image
        let resized = img.resize_exact(new_width, new_height, FilterType::Lanczos3);

        // Save the resized image over the original image
        resized.save_with_format(file_path, ImageFormat::Jpeg)?;
    }

    Ok(())
}

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

    // resize down to 1280 if needed
    resize_and_overwrite_image(path, 1280)?;

    Ok(())
}

pub async fn resolve_event_picture(event: &mut BaseEvent) -> Option<PathBuf> {
    if let Some(local_path) = event.local_image_path.as_ref() {
        let local_path = get_event_image_path_by_uuid(local_path);
        if local_path.exists() {
            return Some(local_path);
        }
    }

    let events_repo = EVENTS_REPOSITORY.get()?;

    if let Some(image_url) = event.image_url.as_deref() {
        let picture_uuid = Uuid::new_v4();
        let local_path = get_event_image_path_by_uuid(picture_uuid);
        if download_image(image_url, &local_path).await.is_ok() {
            event.local_image_path = Some(picture_uuid.to_string());
            events_repo
                .update_local_image_path(event.id, Some(picture_uuid.to_string()))
                .await
                .ok();

            return Some(local_path);
        }
    }

    None
}
