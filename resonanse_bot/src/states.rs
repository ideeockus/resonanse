use resonanse_common::EventSubjectFilter;

use crate::handlers::FillingEvent;

#[derive(Clone, Default)]
pub enum BaseState {
    #[default]
    Start,
    Idle,
    CreateEvent {
        state: CreateEventState,
        filling_event: FillingEvent,
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
    Name,
    Publicity,
    Description,
    Datetime,
    Geo,
    Subject,
    Picture,
    Finalisation,
}
