use serenity::{
    async_trait,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::{Client, Context, EventHandler},
    model::{
        application::{Command, Interaction},
        gateway::Ready,
    },
    prelude::GatewayIntents,
};

use crate::config::Config;
use crate::commands;

pub struct BotHandler {
    config: Config,
}

#[async_trait]
impl EventHandler for BotHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let response = match command.data.name.as_str() {
                "ping" => {
                    let content = commands::ping::run(&command.data.options());
                    let message = CreateInteractionResponseMessage::new().content(content);
                    CreateInteractionResponse::Message(message)
                }
                "rating" => commands::rating::run(&command.data.options(), &self.config).await,
                "help" => {
                    let content = commands::help::run(&command.data.options());
                    let message = CreateInteractionResponseMessage::new().content(content);
                    CreateInteractionResponse::Message(message)
                }
                _ => {
                    let message = CreateInteractionResponseMessage::new()
                        .content("Commande non implémentée :(");
                    CreateInteractionResponse::Message(message)
                }
            };

            if let Err(why) = command.create_response(&ctx.http, response).await {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("✅ {} est connecté et prêt!", ready.user.name);
        println!("🆔 ID du bot: {}", ready.user.id);

        // Enregistrer les commandes globales
        let commands = Command::set_global_commands(&ctx.http, vec![
            commands::ping::register(),
            commands::rating::register(),
            commands::help::register(),
        ]).await;

        println!("✅ Commandes globales enregistrées: {commands:#?}");
    }
}

pub struct Bot {
    client: Client,
}

impl Bot {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        // Créer le client avec les intents nécessaires
        let intents = GatewayIntents::empty(); // Pas besoin d'intents pour les slash commands
        
        let client = Client::builder(&config.discord_token, intents)
            .event_handler(BotHandler { config })
            .await?;
        
        Ok(Bot { client })
    }
    
    pub async fn start(&mut self) -> anyhow::Result<()> {
        println!("🚀 Bot en cours de démarrage...");
        
        if let Err(why) = self.client.start().await {
            println!("❌ Erreur lors du démarrage du client: {:?}", why);
            return Err(anyhow::anyhow!("Erreur de démarrage: {:?}", why));
        }
        
        Ok(())
    }
}
