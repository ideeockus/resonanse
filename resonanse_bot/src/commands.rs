use teloxide::utils::command::{BotCommands, ParseError};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "snake_case",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Начать")]
    Start,
    #[command(description = "Описание")]
    About,
    #[command(description = "Создать")]
    CreateEvent,
    #[command(description = "Список")]
    GetEvents,
    #[command(description = "Обратная связь")]
    SendFeedback,
    // #[command(description = "Выбор события", parse_with = accept_two_digits)]
    // Event{event_num: i64},
}

