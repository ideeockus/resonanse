use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Помогите")]
    Help,
    #[command(description = "Создать")]
    CreateEvent,
    #[command(description = "Список")]
    GetEvents,
    #[command(description = "Обратная связь")]
    SendFeedback,
}
