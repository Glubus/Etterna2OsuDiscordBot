use rosu_map::Beatmap;
use anyhow::Result;
use reqwest;


// link needed
// https://osu.ppy.sh/osu/1234567890
//
pub async fn load_beatmap_from_link(osu_link: &str) -> Result<Beatmap> {
    let response = reqwest::get(osu_link).await?;
    let content = response.text().await?;
    let beatmap = Beatmap::from_bytes(content.as_bytes())?;
    Ok(beatmap)
}



// TODO: remove this if not used anymore
// pub async fn get_calc(beatmap: &Beatmap) -> Result<PatternsValues> {
//     let patterns = transform_to_hit_objects(beatmap);
//     let patterns_values = patterns.get_patterns_values();
//     Ok(patterns_values)
// }
    
