use crate::data_structs::FillingEvent;
use resonanse_common::EventSubjectFilter;
use teloxide::types::MessageId;

#[derive(Clone, Default)]
pub enum BaseState {
    #[default]
    Start,
    Idle,
    CreateEvent {
        state: CreateEventState,
        filling_event: FillingEvent,
        last_edit_msg_id: MessageId,
    },
    GetEventList {
        page_size: i64,
        page_num: i64,
        events_filter: EventSubjectFilter,
    },
    SendFeedback,
}

#[derive(Clone, Debug, Default)]
pub enum CreateEventState {
    #[default]
    Idle,
    EventKind,
    EventTitle,
    Description,
    BriefDescription,
    DatetimeFrom,
    DatetimeTo,
    Geo,
    PlaceTitle,
    Subject,
    Picture,
    ContactInfo,
    Finalisation,
}
