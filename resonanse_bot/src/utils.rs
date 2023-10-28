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

#[inline]
pub fn repr_user_as_str(user: Option<&teloxide::types::User>) -> String {
    match user {
        None => "Unknown user".to_string(),
        Some(user) => {
            format!(
                "{} {} {} [{}]",
                user.first_name,
                user.last_name.as_ref().unwrap_or(&String::new()),
                user.username.as_ref().and_then(|username| {
                    Some(format!("@{}", username))
                }).unwrap_or(String::new()),
                user.id,
            )
        }
    }
}