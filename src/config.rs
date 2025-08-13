use std::env;
use anyhow::Result;
use crate::services::OsuApiService;

pub struct Config {
    pub discord_token: String,
    pub osu_service: OsuApiService,
}

impl Config {
    pub async fn load() -> Result<Self> {
        dotenv::dotenv().ok();
        
        let discord_token = env::var("DISCORD_TOKEN")
            .expect("DISCORD_TOKEN doit être défini dans le fichier .env");
        
        
        let osu_client_id = env::var("OSU_CLIENT_ID")
            .expect("OSU_CLIENT_ID doit être défini dans le fichier .env");
        
        let osu_client_secret = env::var("OSU_CLIENT_SECRET")
            .expect("OSU_CLIENT_SECRET doit être défini dans le fichier .env");
        
        // Initialiser le service OSU API
        let osu_service = OsuApiService::new(
            osu_client_id.parse::<u64>()?,
            osu_client_secret.clone(),
        ).await?;
        
        Ok(Config {
            discord_token,
            osu_service,
        })
    }
}
