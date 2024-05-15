use std::collections::HashMap;

use chrono::NaiveDateTime;
use log::debug;
use serde::Deserialize;
use serde_json::Value;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
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

#[derive(Clone, Debug, Deserialize)]
pub struct Venue {
    pub title: Option<String>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl Venue {
    pub fn from_ll(latitude: f64, longitude: f64) -> Self {
        Self {
            title: None,
            address: None,
            latitude: Some(latitude),
            longitude: Some(longitude),
        }
    }

    pub fn get_yandex_map_link_to(&self) -> Option<String> {
        Some(format!(
            "https://yandex.ru/maps/?pt={},{}&z=15",
            self.longitude?, self.latitude?
        ))
    }

    pub fn parse_from_yandex_map_link(link_str: &str) -> Option<Self> {
        let url = url::Url::parse(link_str).ok();
        Venue::parse_from_yandex_map_url(url.as_ref())
    }

    pub fn parse_from_yandex_map_url(map_url: Option<&url::Url>) -> Option<Self> {
        let (_key, value) = map_url?.query_pairs().find(|(k, _v)| k == "ll")?;

        debug!("parsed from link: k {} : v {}", _key, value);
        let mut split = value.splitn(2, ',');
        let longitude = split.next().and_then(|s| s.parse::<f64>().ok())?;
        let latitude = split.next().and_then(|s| s.parse::<f64>().ok())?;

        Some(Self {
            title: Some(String::new()),
            address: Some(String::new()),
            latitude: Some(latitude),
            longitude: Some(longitude),
        })
    }

    pub fn get_name(&self) -> String {
        match (self.title.as_deref(), self.address.as_deref()) {
            (None, None) => "".to_string(),
            (None, Some(addr)) => addr.to_string(),
            (Some(title), None) => title.to_string(),
            (Some(tittle), Some(addr)) => format!("{}, {}", tittle, addr,),
        }
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

impl From<EventSubject> for String {
    fn from(value: EventSubject) -> Self {
        value.to_string()
    }
}

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

#[derive(Clone, Debug, Deserialize)]
pub struct BaseEvent {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub datetime_from: NaiveDateTime,
    pub datetime_to: Option<NaiveDateTime>,
    pub city: Option<String>,
    pub venue: Venue,
    pub image_url: Option<String>,
    pub local_image_path: Option<String>,
    pub price_price: Option<f32>,
    pub price_currency: Option<String>,
    pub tags: Option<Vec<String>>,
    pub contact: Option<String>,
    pub service_id: String,
    pub service_type: Option<String>,
    pub service_data: Option<Value>,
}

impl BaseEvent {
    pub fn get_description_up_to(&self, n: usize) -> String {
        let description = self.description.as_deref().unwrap_or_default();
        let n = std::cmp::min(n, description.chars().count());
        // format!("{}...", &description[0..n])

        let mut stripped_description = String::new();
        for (i, ch) in  description.chars().enumerate() {
            stripped_description.push(ch);
            if i >= n {
                break;
            }
        }

        stripped_description
    }
}

impl FromRow<'_, PgRow> for BaseEvent {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::error::Error> {
        // let price_price: Option<f32> = row.try_get("price_price")?;
        // let price_currency: Option<String> = row.try_get("price_currency")?;
        //
        // let price = match (price_price, price_currency) {
        //     (Some(price), Some(currency)) => {
        //         Some(crate::models::events::Price {
        //             price,
        //             currency,
        //         })
        //     },
        //     _ => None,
        // };

        Ok(Self {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            datetime_from: row.try_get("datetime_from")?,
            datetime_to: row.try_get("datetime_to")?,
            city: row.try_get("city")?,
            venue: Venue {
                title: row.try_get("venue_title")?,
                address: row.try_get("venue_address")?,
                latitude: row.try_get("venue_lat")?,
                longitude: row.try_get("venue_lon")?,
            },
            image_url: row.try_get("image_url")?,
            local_image_path: row.try_get("local_image_path")?,
            price_price: row.try_get("price_price")?,
            price_currency: row.try_get("price_currency")?,
            tags: row.try_get("tags")?,
            contact: row.try_get("contact")?,
            service_id: row.try_get("service_id")?,
            service_type: row.try_get("service_type")?,
            service_data: row.try_get("service_data")?,
        })
    }
}
