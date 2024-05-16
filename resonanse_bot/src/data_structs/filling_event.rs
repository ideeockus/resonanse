use serde_json::json;
use uuid::Uuid;

use crate::errors::BotHandlerError;
use resonanse_common::models::{BaseEvent, EventSubject, ResonanseEventKind, Venue};
// use resonanse_common::repository::CreateBaseEvent;

#[derive(Clone, Default)]
/// This struct is used during event filling process
pub struct FillingEvent {
    pub title: Option<String>,
    pub event_kind: ResonanseEventKind,
    pub subject: Option<EventSubject>,
    pub description: Option<String>,
    pub brief_description: Option<String>,
    pub datetime_from: Option<chrono::NaiveDateTime>,
    pub datetime_to: Option<chrono::NaiveDateTime>,
    pub city: Option<String>,
    pub geo_position: Option<Venue>,
    pub location_title: Option<String>,
    pub picture: Option<Uuid>,
    pub contact_info: Option<String>,
    pub creator_id: i64,
}

impl FillingEvent {
    pub fn new(city: Option<String>, creator_id: i64) -> Self {
        FillingEvent {
            title: None,
            event_kind: ResonanseEventKind::UserOffer,
            subject: None,
            description: None,
            brief_description: None,
            datetime_from: None,
            datetime_to: None,
            city,
            geo_position: None,
            location_title: None,
            picture: None,
            contact_info: None,
            creator_id,
        }
    }

    pub fn is_ready(&self) -> bool {
        [
            self.title.is_some(),
            self.description.is_some(),
            self.subject.is_some(),
            self.datetime_from.is_some(),
            self.location_title.is_some(),
            self.picture.is_some(),
            self.contact_info.is_some(),
        ]
        .into_iter()
        .all(|is_field_some| is_field_some)
    }

    pub fn get_missed_data_hint(&self) -> String {
        if self.is_ready() {
            return "Данные готовы, но превью почему-то не отоброжается\
            Пожалуйста, сообщите об этом через /send_feedback"
                .to_string();
        }

        let mut missed_data_hint =
            "Чтобы увидеть превью, нужно указать следующие данные:\n".to_string();
        for (is_field_missed, hint_text) in [
            (self.title.is_none(), "Название"),
            (self.description.is_none(), "Описание"),
            (self.datetime_from.is_none(), "Дата и время начала"),
            (self.location_title.is_none(), "Название места"),
            (self.contact_info.is_none(), "Организатор"),
        ]
        .into_iter()
        {
            if is_field_missed {
                missed_data_hint.push_str("\n[ ]\\-");
            } else {
                missed_data_hint.push_str("\n[✅]\\-");
            }
            missed_data_hint.push_str(hint_text);
        }

        missed_data_hint
    }
}

impl TryFrom<FillingEvent> for BaseEvent {
    type Error = BotHandlerError;

    fn try_from(value: FillingEvent) -> Result<Self, Self::Error> {
        Ok(BaseEvent {
            id: Uuid::nil(),
            title: value.title.ok_or(BotHandlerError::UnfilledEvent)?,
            description: Some(value.description.ok_or(BotHandlerError::UnfilledEvent)?),
            datetime_from: value.datetime_from.ok_or(BotHandlerError::UnfilledEvent)?,
            datetime_to: value.datetime_to,
            city: value.city,
            venue: Venue {
                title: Some(value.location_title.ok_or(BotHandlerError::UnfilledEvent)?),
                address: None,
                latitude: value.geo_position.clone().and_then(|v| v.latitude),
                longitude: value.geo_position.clone().and_then(|v| v.longitude),
            },
            image_url: None,
            local_image_path: value.picture.map(|v| v.to_string()),
            price: None,
            tags: None,
            contact: value.contact_info,
            service_id: "resonanse_0".to_string(), // this is fake service_id
            service_type: Some("RESONANSE".to_string()),
            service_data: Some(json!({
                "creator_id": value.creator_id,
                "creation_time": chrono::offset::Local::now().naive_local(),
            })),
        })
    }
}
