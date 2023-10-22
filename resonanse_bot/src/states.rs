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
        page_size: usize,
        list_page: usize,
    },
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