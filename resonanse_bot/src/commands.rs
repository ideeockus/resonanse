use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "snake_case",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Начать")]
    Start,
    #[command(description = "Описание")]
    Help,
    #[command(description = "Обратная связь")]
    SendFeedback,

    #[command(description = "Создать")]
    CreateEvent,
    #[command(description = "Список")]
    GetEvents,

    #[command(description = "Выбрать город")]
    City,
    #[command(description = "Поставить описание пользователя")]
    SetDescription,
    #[command(description = "Запрос дайджеста")]
    GetDigest,

    #[command(description = "Запусить WebApp")]
    RunWebApp,
    #[command(description = "Внести donation")]
    SendDonation,
}
