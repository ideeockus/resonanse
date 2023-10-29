use std::env;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use crate::configuration::RESONANSE_STORAGE_DIR;

const BASE_STORAGE_DIR_NAME: &str = "resonanse_storage";
const EVENT_IMAGES_DIR_NAME: &str = "event_images";

// static EVENT_IMAGES_PATH: PathBuf = get_event_images_path();

pub fn get_event_images_path() -> PathBuf {
    let resonanse_base_dir = env::var(RESONANSE_STORAGE_DIR);
    let resonanse_base_dir = resonanse_base_dir
        .as_deref()
        .unwrap_or(".");
    let path = Path::new(resonanse_base_dir)
        .join(BASE_STORAGE_DIR_NAME)
        .join(EVENT_IMAGES_DIR_NAME);

    if !path.exists() {
        create_dir_all(&path).unwrap();
    }

    path
}