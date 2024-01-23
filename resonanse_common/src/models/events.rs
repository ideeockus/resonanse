use std::collections::HashMap;

use chrono::NaiveDateTime;
use log::debug;
use sqlx::{FromRow, Row};
use sqlx::postgres::PgRow;
use strum_macros;
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
}

impl Location {
    pub fn from_ll(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    pub fn try_from_ll(latitude: Option<f64>, longitude: Option<f64>) -> Option<Self> {
        let latitude = latitude?;
        let longitude = longitude?;

        Some(Self {
            latitude,
            longitude,
        })
    }

    pub fn get_yandex_map_link_to(&self) -> String {
        format!(
            "https://yandex.ru/maps/?pt={},{}&z=15",
            self.longitude, self.latitude
        )
    }

    pub fn parse_from_yandex_map_link(link_str: &str) -> Option<Self> {
        let url = url::Url::parse(link_str).ok();
        Location::parse_from_yandex_map_url(url.as_ref())
    }

    pub fn parse_from_yandex_map_url(map_url: Option<&url::Url>) -> Option<Self> {
        let (_key, value) = map_url?.query_pairs().find(|(k, _v)| k == "ll")?;

        debug!("parsed from link: k {} : v {}", _key, value);
        let mut split = value.splitn(2, ',');
        let longitude = split.next().and_then(|s| s.parse::<f64>().ok())?;
        let latitude = split.next().and_then(|s| s.parse::<f64>().ok())?;

        Some(Self {
            latitude,
            longitude,
        })
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    sqlx::Type,
    Eq,
    Hash,
    PartialEq,
    strum_macros::EnumString,
    strum_macros::Display,
)]
#[repr(i32)]
pub enum EventSubject {
    #[strum(serialize = "event_subject.other")]
    Other = 0,
    #[strum(serialize = "event_subject.professional")]
    Professional = 1,
    #[strum(serialize = "event_subject.business")]
    Business = 2,
    #[strum(serialize = "event_subject.education")]
    Education = 3,
    #[strum(serialize = "event_subject.entertainments")]
    Entertainments = 4,
    #[strum(serialize = "event_subject.sport")]
    Sport = 5,
    #[strum(serialize = "event_subject.social")]
    Social = 6,
    #[strum(serialize = "event_subject.culture")]
    Culture = 7,
    #[strum(serialize = "event_subject.charity")]
    Charity = 8,
}

// impl Display for EventSubject {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let s = match self {
//             EventSubject::Business => "Бизнес",
//             EventSubject::Social => "Знакомства",
//             EventSubject::Sport => "Спорт",
//             EventSubject::Charity => "Добро",
//             EventSubject::Education => "Образование",
//             EventSubject::Professional => "Профессия",
//             EventSubject::Entertainments => "Развлечения",
//             EventSubject::Culture => "Культура",
//             // EventSubject::Interests => "Интересы",
//             EventSubject::Other => "Другое",
//         };
//         write!(f, "{}", s)
//     }
// }

impl From<EventSubject> for String {
    fn from(value: EventSubject) -> Self {
        value.to_string()
    }
}

// impl From<&str> for EventSubject {
//     fn from(value: &str) -> Self {
//         match value {
//             "Бизнес" => EventSubject::Business,
//             "Знакомства" => EventSubject::Social,
//             "Спорт" => EventSubject::Sport,
//             "Добро" => EventSubject::Charity,
//             "Образование" => EventSubject::Education,
//             "Профессия" => EventSubject::Professional,
//             "Развлечения" => EventSubject::Entertainments,
//             "Культура" => EventSubject::Culture,
//             // "Интересы" => EventSubject::Interests,
//             _ => EventSubject::Other,
//         }
//     }
// }

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
            (EventSubject::Other, true),
        ]))
    }

    pub fn switch(&mut self, event_subject: EventSubject) {
        if let Some(f) = self.0.get_mut(&event_subject) {
            *f = !*f;
        }
    }

    pub fn get_filters(&self) -> HashMap<EventSubject, bool> {
        let mut filters = self.0.clone();
        filters.remove(&EventSubject::Other);
        filters
    }
}

impl Default for EventSubjectFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    sqlx::Type,
    Eq,
    Hash,
    PartialEq,
    strum_macros::EnumString,
    strum_macros::Display,
)]
#[repr(i32)]
/// Kind of resonanse event
pub enum ResonanseEventKind {
    #[strum(serialize = "event_kind.announcement")]
    Announcement = 0,
    #[strum(serialize = "event_kind.user_offer")]
    UserOffer = 1,
    // Private = 2,
}

impl Default for ResonanseEventKind {
    fn default() -> Self {
        Self::UserOffer
    }
}

// impl MyI18N for ResonanseEventKind {
//     fn to_text(&self) -> &'static str {
//         match self {
//             ResonanseEventKind::Announcement => {}
//             ResonanseEventKind::UserOffer => {}
//         }
//     }
//
//     fn from_text(text: &str) -> Self {
//         todo!()
//     }
// }

// const EVENT_KIND_ANNOUNCEMENT: &str = "Announcement";
// const EVENT_KIND_USER_OFFER: &str = "UserOffer";
// impl Display for EventSubject {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let s = match self {
//             EventSubject::Business => "Бизнес",
//             EventSubject::Social => "Знакомства",
//             EventSubject::Sport => "Спорт",
//             EventSubject::Charity => "Добро",
//             EventSubject::Education => "Образование",
//             EventSubject::Professional => "Профессия",
//             EventSubject::Entertainments => "Развлечения",
//             EventSubject::Culture => "Культура",
//             // EventSubject::Interests => "Интересы",
//             EventSubject::Other => "Другое",
//         };
//         write!(f, "{}", s)
//     }
// }
//
// impl From<&str> for EventSubject {
//     fn from(value: &str) -> Self {
//         match value {
//             "Бизнес" => EventSubject::Business,
//             "Знакомства" => EventSubject::Social,
//             "Спорт" => EventSubject::Sport,
//             "Добро" => EventSubject::Charity,
//             "Образование" => EventSubject::Education,
//             "Профессия" => EventSubject::Professional,
//             "Развлечения" => EventSubject::Entertainments,
//             "Культура" => EventSubject::Culture,
//             // "Интересы" => EventSubject::Interests,
//             _ => EventSubject::Other,
//         }
//     }
// }

// todo translation
// const EVENT_SUBJECTS: &[&str] = &[
//     "Бизнес",
//     "Спорт",
//     "Благотворительность",
//     "Развлечения",
//     "Образование",
//     "Профессиональное",
//     "Знакомства",
//     "Культура",
//     "Интересы",
//     "Другое",
// ];

#[derive(Clone, Debug)]
pub struct BaseEvent {
    pub id: Uuid,
    pub is_private: bool,
    pub is_commercial: bool,
    pub event_kind: ResonanseEventKind,
    pub title: String,
    pub description: String,
    pub brief_description: Option<String>,
    // markdown (?)
    pub subject: EventSubject,
    pub datetime_from: NaiveDateTime,
    pub datetime_to: Option<NaiveDateTime>,
    // pub timezone: chrono_tz::Tz,
    pub location: Option<Location>,
    pub location_title: String,
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
            event_kind: row.try_get::<_, &str>("event_kind")?,
            title: row.try_get::<_, &str>("title")?,
            description: row.try_get::<_, &str>("description")?,
            brief_description: row.try_get::<_, &str>("brief_description")?,
            subject: row.try_get::<_, &str>("subject")?,
            datetime_from: row.try_get::<_, &str>("datetime_from")?,
            datetime_to: row.try_get::<_, &str>("datetime_to")?,
            location: Location::try_from_ll(
                row.try_get::<_, &str>("location_latitude")?,
                row.try_get::<_, &str>("location_longitude")?,
            ),
            location_title: row.try_get::<_, &str>("location_title")?,
            creator_id: row.try_get::<_, &str>("creator_id")?,
            event_type: row.try_get::<_, &str>("event_type")?,
            picture: row.try_get::<_, &str>("picture")?,
            creation_time: row.try_get::<_, &str>("creation_time")?,
            contact_info: row.try_get::<_, &str>("contact_info")?,
        })
    }
}
