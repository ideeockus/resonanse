use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use sqlx::{FromRow, Postgres, Row, Type};
use sqlx::postgres::PgRow;
use uuid::Uuid;
// use serde_repr::{Serialize_repr, Deserialize_repr};


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
        format!("https://yandex.ru/maps/?pt={},{}&z=15", self.longitude, self.latitude)
    }
}

#[derive(Clone, Copy, Debug, sqlx::Type)]
#[repr(i32)]
pub enum EventSubject {
    Other = 0,
    Social = 1,
    Sport = 2,
    Charity = 3,
    Education = 4,
    Professional = 5,
    Acquaintance = 6,
    Culture = 7,
    Interests = 8,
    Business = 9,
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
}

impl FromRow<'_, PgRow> for BaseEvent {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::error::Error> {
        Ok(
            Self {
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
            }
        )
    }
}