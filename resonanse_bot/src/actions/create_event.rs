use resonanse_common::models::Location;

#[derive(Clone, Default)]
pub struct FillingEvent {
    title: Option<String>,
    is_private: bool,
    subject: Option<String>,
    description: Option<String>,
    datetime: Option<chrono::DateTime<chrono::Utc>>,
    geo_position: Option<Location>,
    photo: Option<String>,
}


