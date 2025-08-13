use serenity::builder::CreateCommand;

pub fn register() -> CreateCommand {
    CreateCommand::new("help")
        .description("Affiche l'aide avec toutes les commandes")
        .description_localized("en-US", "Show help with all commands")
}

pub fn run(_options: &[serenity::model::application::ResolvedOption]) -> String {
    r#"
**Commandes disponibles:**

`/calc <link>` - Traite un lien osu.ppy.sh
  Exemple: `/calc https://osu.ppy.sh/beatmapsets/12345`

`/ping` - Test de latence du bot

`/help` - Affiche cette aide
"#.to_string()
}
