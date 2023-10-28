use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "snake_case",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Описание")]
    Help,
    #[command(description = "Создать")]
    CreateEvent,
    #[command(description = "Список")]
    GetEvents,
    #[command(description = "Обратная связь")]
    SendFeedback,
}
