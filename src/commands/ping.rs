use serenity::builder::CreateCommand;

pub fn register() -> CreateCommand {
    CreateCommand::new("ping")
        .description("Test de latence du bot")
        .description_localized("en-US", "Bot latency test")
}

pub fn run(_options: &[serenity::model::application::ResolvedOption]) -> String {
    "Pong! 🏓".to_string()
}
