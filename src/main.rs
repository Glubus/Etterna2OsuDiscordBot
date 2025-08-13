use anyhow::Result;

mod bot;
mod config;
mod commands;
mod handlers;
mod services;
mod helpers;
mod embed;

use bot::Bot;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🤖 Démarrage du bot Discord...");
    
    // Charger la configuration
    let config = Config::load().await?;
    println!("📋 Configuration chargée");
    
    // Créer et démarrer le bot
    let mut bot = Bot::new(config).await?;
    bot.start().await?;
    
    Ok(())
}
