use serenity::builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage, CreateAttachment};
use serenity::model::application::CommandOptionType;
use crate::handlers::osu_handler::OsuHandler;
use crate::config::Config;

pub fn register() -> CreateCommand {
    CreateCommand::new("rating")
        .description("Charge une beatmap à partir d'un lien osu.ppy.sh (TODO: traitement)")
        .description_localized("en-US", "Load a beatmap from an osu.ppy.sh link (TODO: processing)")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "link",
                "Le lien osu.ppy.sh à traiter"
            )
            .description_localized("en-US", "The osu.ppy.sh link to process")
            .required(true)
        )
}

pub async fn run(options: &[serenity::model::application::ResolvedOption<'_>], config: &Config) -> CreateInteractionResponse {
    // Récupérer le lien depuis les options
    let link_option = options.iter().find(|opt| opt.name == "link");
    
    match link_option {
        Some(opt) => {
            if let serenity::model::application::ResolvedValue::String(link) = opt.value {
                // Vérifier que c'est bien un lien osu.ppy.sh
                if link.contains("osu.ppy.sh") {
                    // Utiliser la fonction rating qui charge la beatmap
                    match OsuHandler::rating(&link, &config.osu_service).await {
                        Ok((embed, chart_bytes)) => {
                            let file = CreateAttachment::bytes(chart_bytes, "hexagonal.png");
                            let message = CreateInteractionResponseMessage::new()
                                .embed(embed)
                                .add_file(file);
                            CreateInteractionResponse::Message(message)
                        }
                        Err(e) => {
                            let message = CreateInteractionResponseMessage::new()
                                .content(format!("❌ Erreur lors du traitement: {}", e));
                            CreateInteractionResponse::Message(message)
                        }
                    }
                } else {
                    let message = CreateInteractionResponseMessage::new()
                        .content("❌ Erreur: Veuillez fournir un lien osu.ppy.sh valide.");
                    CreateInteractionResponse::Message(message)
                }
            } else {
                let message = CreateInteractionResponseMessage::new()
                    .content("❌ Erreur: le lien doit être une chaîne de caractères.");
                CreateInteractionResponse::Message(message)
            }
        }
        None => {
            let message = CreateInteractionResponseMessage::new()
                .content("❌ Veuillez fournir un lien osu.ppy.sh.");
            CreateInteractionResponse::Message(message)
        }
    }
}
