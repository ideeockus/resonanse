use crate::commands::Command;
use crate::handlers::*;
use crate::states::*;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::prelude::*;

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        // .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Start].endpoint(start_command))
        .branch(case![Command::About].endpoint(about_command))
        .branch(case![Command::CreateEvent].endpoint(create_event_command))
        .branch(case![Command::GetEvents].endpoint(get_events_command))
        .branch(case![Command::SendFeedback].endpoint(send_feedback_command));

    let message_handler = Update::filter_message()
        .map_async(log_msg_handler)
        .branch(command_handler)
        .branch(case![BaseState::Start].endpoint(handle_start_state))
        .branch(case![BaseState::SendFeedback].endpoint(handle_send_feedback))
        .branch(
            case![BaseState::GetEventList {
                page_size,
                page_num,
                events_filter,
            }]
            .endpoint(handle_get_events),
        )
        .branch(
            case![BaseState::CreateEvent {
                state,
                filling_event,
                last_edit_msg_id,
            }]
            .endpoint(handle_create_event_state_message),
        )
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query()
        .map_async(log_callback_handler)
        .branch(
            case![BaseState::CreateEvent {
                state,
                filling_event,
                last_edit_msg_id
            }]
            .endpoint(handle_create_event_state_callback),
        )
        .branch(
            case![BaseState::GetEventList {
                page_size,
                page_num,
                events_filter,
            }]
            .endpoint(handle_get_events_callback),
        )
        .branch(dptree::endpoint(invalid_state_callback));

    dialogue::enter::<Update, InMemStorage<BaseState>, BaseState, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}
