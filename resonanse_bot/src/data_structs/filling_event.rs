use uuid::Uuid;

use crate::errors::BotHandlerError;
use resonanse_common::models::{BaseEvent, EventSubject, Location, ResonanseEventKind};
// use resonanse_common::repository::CreateBaseEvent;

#[derive(Clone, Default)]
/// This struct is used during event filling process
pub struct FillingEvent {
    pub title: Option<String>,
    pub is_private: bool,
    pub event_kind: ResonanseEventKind,
    pub subject: Option<EventSubject>,
    pub description: Option<String>,
    pub brief_description: Option<String>,
    pub datetime_from: Option<chrono::NaiveDateTime>,
    pub datetime_to: Option<chrono::NaiveDateTime>,
    pub geo_position: Option<Location>,
    pub location_title: Option<String>,
    pub picture: Option<Uuid>,
    pub contact_info: Option<String>,
    pub creator_id: i64,
}

impl FillingEvent {
    pub fn new() -> Self {
        FillingEvent {
            title: None,
            is_private: false,
            event_kind: ResonanseEventKind::UserOffer,
            subject: None,
            description: None,
            brief_description: None,
            datetime_from: None,
            datetime_to: None,
            geo_position: None,
            location_title: None,
            picture: None,
            contact_info: None,
            creator_id: 0,
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
            (self.subject.is_none(), "Тематика"),
            (self.datetime_from.is_none(), "Дата и время начала"),
            (self.location_title.is_none(), "Название места"),
            (self.picture.is_none(), "Изображение или постер"),
            (self.contact_info.is_none(), "Контакт для связи"),
        ]
        .into_iter()
        {
            if is_field_missed {
                missed_data_hint.push_str("\n\\-");
                missed_data_hint.push_str(hint_text);
            }
        }

        missed_data_hint
    }
}

// impl From<FillingEvent> for BaseEvent {
//     fn from(value: FillingEvent) -> Self {
//         BaseEvent {
//             id: Uuid::nil(),
//             is_private: false,
//             is_commercial: false,
//             event_kind: value.event_kind,
//             title: value.title.unwrap_or("No title".to_string()),
//             description: value.description.unwrap_or("No description".to_string()),
//             brief_description: value.brief_description,
//             subject: value.subject.unwrap_or(EventSubject::Other),
//             datetime_from: value
//                 .datetime_from
//                 .unwrap_or(chrono::Local::now().naive_local()),
//             // timezone: chrono_tz::Tz::Europe__Moscow,
//             datetime_to: value.datetime_to,
//             location: value.geo_position,
//             location_title: value.location_title.unwrap_or("No place".to_string()),
//             creator_id: value.creator_id,
//             event_type: Default::default(),
//             picture: value.picture,
//             // creation_time: Default::default(),
//             creation_time: chrono::offset::Local::now().naive_local(),
//             contact_info: value.contact_info,
//         }
//     }
// }

impl TryFrom<FillingEvent> for BaseEvent {
    type Error = BotHandlerError;

    fn try_from(value: FillingEvent) -> Result<Self, Self::Error> {
        Ok(BaseEvent {
            id: Uuid::nil(),
            is_private: false,
            is_commercial: false,
            event_kind: value.event_kind,
            title: value.title.ok_or(BotHandlerError::UnfilledEvent)?,
            description: value.description.ok_or(BotHandlerError::UnfilledEvent)?,
            brief_description: value.brief_description,
            subject: value.subject.ok_or(BotHandlerError::UnfilledEvent)?,
            datetime_from: value.datetime_from.ok_or(BotHandlerError::UnfilledEvent)?,
            // timezone: chrono_tz::Tz::Europe__Moscow,
            datetime_to: value.datetime_to,
            location: value.geo_position,
            location_title: value.location_title.ok_or(BotHandlerError::UnfilledEvent)?,
            creator_id: value.creator_id,
            event_type: Default::default(),
            picture: value.picture,
            // creation_time: Default::default(),
            creation_time: chrono::offset::Local::now().naive_local(),
            contact_info: value.contact_info,
        })
    }
}

// impl From<BaseEvent> for FillingEvent {
//     fn from(value: BaseEvent) -> Self {
//         Self {
//             title: Some(value.title),
//             is_private: value.is_private,
//             event_kind: value.event_kind,
//             subject: Some(value.subject),
//             description: None,
//             brief_description: None,
//             datetime_from: None,
//             datetime_to: None,
//             geo_position: None,
//             location_title: None,
//             picture: None,
//             contact_info: None,
//             creator_id: 0,
//         }
//     }
// }
