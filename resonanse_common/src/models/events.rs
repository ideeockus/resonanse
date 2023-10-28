use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use uuid::Uuid;

#[derive(Clone, Copy, Debug)]
pub enum EventType {
    OfflineMeetup,
    OneToOne,
    Online,
    Unknown,
}

impl Default for EventType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Clone, Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub title: Option<String>,
    pub address: Option<String>,
}

impl Location {
    pub fn from_ll(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            title: None,
            address: None,
        }
    }

    pub fn get_yandex_map_link_to(&self) -> String {
        format!("https://yandex.ru/maps/?pt={},{}&z=15", self.longitude, self.latitude)
    }
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

impl From<&str> for EventSubject {
    fn from(value: &str) -> Self {
        match value {
            "Бизнес" => EventSubject::Business,
            "Спорт" => EventSubject::Social,
            "Благотворительность" => EventSubject::Sport,
            "Развлечения" => EventSubject::Charity,
            "Образование" => EventSubject::Education,
            "Профессиональное" => EventSubject::Professional,
            "Знакомства" => EventSubject::Acquaintance,
            "Культура" => EventSubject::Culture,
            "Интересы" => EventSubject::Interests,
            _ => EventSubject::Other,
        }
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
pub struct BaseEvent {
    pub id: u64,
    pub is_private: bool,
    pub is_commercial: bool,
    pub title: String,
    pub description: String,
    // markdown (?)
    pub subject: EventSubject,
    pub datetime: NaiveDateTime,
    pub timezone: chrono_tz::Tz,
    pub location: Location,
    pub creator_id: u64,
    pub event_type: EventType,
    pub picture: Uuid,
}
