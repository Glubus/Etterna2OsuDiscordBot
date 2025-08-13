use anyhow::Result;

use crate::helpers::link::transform_link;
use serenity::builder::{CreateEmbed};
use crate::services::etterna_rating;
use crate::helpers::osu::load_beatmap_from_link;
use crate::services::OsuApiService;
use crate::services::chart::generate_msd_chart;
use crate::embed::rating::rating_embed;
use tokio::try_join;

pub struct OsuHandler;
impl OsuHandler {

    

    pub async fn rating(
        beatmapset_link: &str,
        osu_api: &OsuApiService,
    ) -> Result<(CreateEmbed, Vec<u8>)> {
    
    
        // step 1 : transform_link
        let (beatmap_id, osu_link) = transform_link(beatmapset_link)?;
    
        // step 2 : parallel fetch beatmap and API call
        let (beatmap, beatmap_data) = try_join!(
            load_beatmap_from_link(&osu_link),
            osu_api.get_beatmap(beatmap_id)
        )?;
    
        // step 3 : calculate msd
        let msd = etterna_rating::calculate_msd(&beatmap)
            .map_err(|e| anyhow::anyhow!("Error calculating MSD: {}", e))?;
    
        // step 4 : get beatmapset
        let beatmapset = beatmap_data
            .mapset
            .ok_or_else(|| anyhow::anyhow!("Beatmapset not found"))?;
    
        let creator = beatmapset.creator_name;
        let embed_fut = rating_embed(
            beatmapset_link,
            &beatmapset.artist,
            &beatmapset.title,
            &beatmapset.covers.card_2x,
            &creator,
            beatmap.overall_difficulty,
            beatmap_data.seconds_drain,
            beatmap_data.bpm,
            beatmap_data.stars,
            &msd,
        );
        
        // step 5 : prepare msd chart
        let chart_fut = tokio::task::spawn_blocking({
            let msd = msd.msds[3].clone();
            let title = beatmapset.title.clone();
            let version = beatmap_data.version.clone();
            move || generate_msd_chart(&msd, &title, &version)
        });
    
        // step 6 : execute parallel
        let embed = embed_fut.await?;
        let chart_bytes = chart_fut.await??;
    
        Ok((embed, chart_bytes))
    }
    
    

    
    
}
