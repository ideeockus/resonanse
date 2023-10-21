pub async fn help(bot: Bot, msg: Message) -> HandlerResult {
    log_request("got help command", &msg);

    let mut message = bot.send_message(msg.chat.id, BOT_HELP_TEXT_MD);
    message.parse_mode = Some(ParseMode::MarkdownV2);
    message.await?;

    Ok(())
}