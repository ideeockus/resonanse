use std::path::PathBuf;
use chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;

pub enum EventType {
    OfflineMeetup,
    OneToOne,
    Online,
}

pub struct Location {
    latitude: f64,
    longitude: f64,
}

pub enum EventSubject {
    Business,
    Social,
    Sport,
    Charity,
    Education,
    Professional,
    Acquaintance,
    Culture,
    Interests,
    Other,
}

// todo translation
const EVENT_SUBJECTS: &[&str] = &[
    "Бизнес",
    "Спорт",
    "Благотворительность",
    "Развлечения",
    "Образование",
    "Профессиональное",
    "Знакомства",
    "Культура",
    "Интересы",
    "Другое",
];

pub struct BaseEvent<Tz: TimeZone = Utc> {
    id: u64,
    is_private: bool,
    is_commercial: bool,
    title: String,
    description: String, // markdown (?)
    subject: EventSubject,
    datetime: DateTime<Tz>,
    location: Location,
    creator_id: u64,
    event_type: EventType,
    picture: Uuid,
}
