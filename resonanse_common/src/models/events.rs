use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use chrono::NaiveDateTime;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, sqlx::Type)]
#[repr(i32)]
pub enum EventType {
    Unknown = 0,
    OfflineMeetup = 1,
    OneToOne = 2,
    Online = 3,
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
}

impl Location {
    pub fn from_ll(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            title: None,
        }
    }

    pub fn get_yandex_map_link_to(&self) -> String {
        format!(
            "https://yandex.ru/maps/?pt={},{}&z=15",
            self.longitude, self.latitude
        )
    }
}

#[derive(Clone, Copy, Debug, sqlx::Type, Eq, Hash, PartialEq)]
#[repr(i32)]
pub enum EventSubject {
    Other = 0,
    Professional = 1,
    Business = 2,
    Education = 3,
    Entertainments = 4,
    Sport = 5,
    Social = 6,
    Culture = 7,
    Charity = 8,
}

impl Display for EventSubject {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EventSubject::Business => "Бизнес",
            EventSubject::Social => "Знакомства",
            EventSubject::Sport => "Спорт",
            EventSubject::Charity => "Добро",
            EventSubject::Education => "Образование",
            EventSubject::Professional => "Профессия",
            EventSubject::Entertainments => "Развлечения",
            EventSubject::Culture => "Культура",
            // EventSubject::Interests => "Интересы",
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
            "Знакомства" => EventSubject::Social,
            "Спорт" => EventSubject::Sport,
            "Добро" => EventSubject::Charity,
            "Образование" => EventSubject::Education,
            "Профессия" => EventSubject::Professional,
            "Развлечения" => EventSubject::Entertainments,
            "Культура" => EventSubject::Culture,
            // "Интересы" => EventSubject::Interests,
            _ => EventSubject::Other,
        }
    }
}

// pub struct EventSubjectFilter(Vec<(EventSubject, bool)>);
#[derive(Clone)]
pub struct EventSubjectFilter(pub HashMap<EventSubject, bool>);

impl EventSubjectFilter {
    pub fn new() -> Self {
        Self(HashMap::from([
            (EventSubject::Business, true),
            (EventSubject::Social, true),
            (EventSubject::Sport, true),
            (EventSubject::Charity, true),
            (EventSubject::Education, true),
            (EventSubject::Professional, true),
            (EventSubject::Entertainments, true),
            (EventSubject::Culture, true),
            // (EventSubject::Interests, true),
            // (EventSubject::Other, true),
        ]))
    }

    pub fn switch(&mut self, event_subject: EventSubject) {
        if let Some(f) = self.0.get_mut(&event_subject) {
            *f = !*f;
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
    pub id: Uuid,
    pub is_private: bool,
    pub is_commercial: bool,
    pub title: String,
    pub description: String,
    // markdown (?)
    pub subject: EventSubject,
    pub datetime: NaiveDateTime,
    // pub timezone: chrono_tz::Tz,
    pub location: Location,
    pub creator_id: i64,
    pub event_type: EventType,
    pub picture: Option<Uuid>,
    pub creation_time: NaiveDateTime,
    pub contact_info: Option<String>,
}

impl FromRow<'_, PgRow> for BaseEvent {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::error::Error> {
        Ok(Self {
            id: row.try_get::<_, &str>("id")?,
            is_private: row.try_get::<_, &str>("is_private")?,
            is_commercial: row.try_get::<_, &str>("is_commercial")?,
            title: row.try_get::<_, &str>("title")?,
            description: row.try_get::<_, &str>("description")?,
            subject: row.try_get::<_, &str>("subject")?,
            datetime: row.try_get::<_, &str>("datetime")?,
            location: Location {
                latitude: row.try_get::<_, &str>("location_latitude")?,
                longitude: row.try_get::<_, &str>("location_longitude")?,
                title: row.try_get::<_, &str>("location_title")?,
            },
            creator_id: row.try_get::<_, &str>("creator_id")?,
            event_type: row.try_get::<_, &str>("event_type")?,
            picture: row.try_get::<_, &str>("picture")?,
            creation_time: row.try_get::<_, &str>("creation_time")?,
            contact_info: row.try_get::<_, &str>("contact_info")?,
        })
    }
}
