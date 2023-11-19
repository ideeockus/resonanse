use log::warn;
use uuid::Uuid;
use resonanse_common::models::{EventSubject, Location};
use resonanse_common::repository::CreateBaseEvent;

#[derive(Clone, Default)]
/// This struct is used during event filling process
pub struct FillingEvent {
    title: Option<String>,
    is_private: bool,
    subject: Option<EventSubject>,
    description: Option<String>,
    datetime: Option<chrono::NaiveDateTime>,
    geo_position: Option<Location>,
    picture: Option<Uuid>,
    contact_info: Option<String>,
    creator_id: i64,
}

impl FillingEvent {
    pub fn new() -> Self {
        FillingEvent {
            title: None,
            is_private: false,
            subject: None,
            description: None,
            datetime: None,
            geo_position: None,
            picture: None,
            contact_info: None,
            creator_id: 0,
        }
    }
}

impl From<FillingEvent> for CreateBaseEvent {
    fn from(value: FillingEvent) -> Self {
        CreateBaseEvent {
            // id: 0,
            is_private: false,
            is_commercial: false,
            title: value.title.unwrap_or("No title".to_string()),
            description: value.description.unwrap_or("No description".to_string()),
            subject: value.subject.unwrap_or(EventSubject::Other),
            datetime: value.datetime.unwrap_or(chrono::Local::now().naive_local()),
            // timezone: chrono_tz::Tz::Europe__Moscow,
            location: value.geo_position.unwrap_or_else(|| {
                warn!("cannot set event for BaseEvent from FillingEvent");
                Location::from_ll(0.0, 0.0)
            }),
            creator_id: value.creator_id,
            event_type: Default::default(),
            picture: value.picture,
            // creation_time: Default::default(),
            contact_info: value.contact_info,
        }
    }
}