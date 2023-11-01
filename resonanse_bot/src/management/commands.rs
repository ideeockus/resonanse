use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "snake_case",
    description = "These commands are supported:"
)]
pub enum ManagementCommand {
    #[command(description = "Удалить эвент")]
    DeleteEvent,
    #[command(description = "Статистика")]
    GetStatistics,
}
