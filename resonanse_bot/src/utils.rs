use std::fs;
use std::path::{Path, PathBuf};

const TG_DOWNLOADS_PATH: &str = "tg_downloads";

pub fn get_tg_downloads_dir() -> PathBuf {
    let tg_downloads_path = Path::new(TG_DOWNLOADS_PATH);
    if !tg_downloads_path.exists() {
        fs::create_dir(tg_downloads_path).expect("Oops, cannot create dir");
    }

    tg_downloads_path.to_path_buf()
}
