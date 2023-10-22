use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use chrono::{DateTime, TimeZone, Utc};
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub enum EventType {
    OfflineMeetup,
    OneToOne,
    Online,
}

#[derive(Clone, Copy, Debug)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}

#[derive(Clone, Copy, Debug)]
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

// impl ToString for EventSubject {
//     fn to_string(&self) -> String {
//         match self {
//             EventSubject::Business => "Бизнес",
//             EventSubject::Social => "Спорт",
//             EventSubject::Sport => "Благотворительность",
//             EventSubject::Charity => "Развлечения",
//             EventSubject::Education => "Образование",
//             EventSubject::Professional => "Профессиональное",
//             EventSubject::Acquaintance => "Знакомства",
//             EventSubject::Culture => "Культура",
//             EventSubject::Interests => "Интересы",
//             EventSubject::Other => "Другое",
//         }.to_string()
//     }
// }

impl Display for EventSubject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EventSubject::Business => "Бизнес",
            EventSubject::Social => "Спорт",
            EventSubject::Sport => "Благотворительность",
            EventSubject::Charity => "Развлечения",
            EventSubject::Education => "Образование",
            EventSubject::Professional => "Профессиональное",
            EventSubject::Acquaintance => "Знакомства",
            EventSubject::Culture => "Культура",
            EventSubject::Interests => "Интересы",
            EventSubject::Other => "Другое",
        };
        write!(f, "{}", s)
    }
}

impl From<EventSubject> for String {
    fn from(value: EventSubject) -> Self {
        value.to_string()
    }
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

#[derive(Clone, Debug)]
pub struct BaseEvent<Tz: TimeZone = Utc> {
    id: u64,
    is_private: bool,
    is_commercial: bool,
    title: String,
    description: String,
    // markdown (?)
    subject: EventSubject,
    datetime: DateTime<Tz>,
    location: Location,
    creator_id: u64,
    event_type: EventType,
    picture: Uuid,
}
