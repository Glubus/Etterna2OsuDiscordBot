use anyhow::Result;
use rosu_v2::prelude::*;
use std::sync::Arc;

pub struct OsuApiService {
    client: Arc<Osu>,
}

impl OsuApiService {
    pub async fn new(client_id: u64, client_secret: String) -> Result<Self> {
        let client = Osu::new(client_id, client_secret).await?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub async fn get_beatmap(&self, beatmap_id: u32) -> Result<BeatmapExtended> {
        let beatmap = self.client.beatmap().map_id(beatmap_id).await?;
        Ok(beatmap)
    } 
}
