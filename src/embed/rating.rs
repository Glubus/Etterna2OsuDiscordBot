use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use anyhow::Result;

pub async fn rating_embed(beatmapset_link: &str, artist: &str, title: &str, cover: &str, creator: &str,
    od: f32,
    hit_length: u32,
    bpm: f32,
    stars: f32,    msd: &minacalc_rs::MsdForAllRates) -> Result<CreateEmbed, anyhow::Error> {
    let mut embed = CreateEmbed::new()
    .title(format!("{} - {}", artist, title))
    .thumbnail(cover)
    .image("attachment://hexagonal.png")
    .color(0xFF69B4)
    .footer(CreateEmbedFooter::new("MSD Calculation"))
    .timestamp(chrono::Utc::now())
    .url(beatmapset_link)
    .field("", format!("Mapped by **{}**", creator), false)
    .field("", format!("OD **`{:.1}`** • Duration **`{}`**\nBPM **`{:.0}`** • SR **`{:.2}`**", od, hit_length, bpm, stars), false);

    for (i, rate) in msd.msds.iter().enumerate() {
        let rate_value = 0.7 + i as f32 * 0.1;

            // Exclure les rates 0.7 (i=0), 0.8 (i=1), et 1.4 (i=7)
        if ![0, 1, 7].contains(&i) && rate_value <= 1.5 {
            embed = embed.field(
                format!("Rate {:.1}", rate_value),
                format!("{:.2}", rate.overall),
                true,
            );
        }
    }

    Ok(embed)
}

    // Crée un embed Discord avec les données de l'API osu
    // pub fn create_beatmap_embed(beatmap: &BeatmapResponse) -> CreateEmbed {
    //     let cover_url = &beatmap.beatmapset.covers.card_2x;
        
    //     CreateEmbed::new()
    //         .title(format!("{} - {}", beatmap.beatmapset.artist, beatmap.beatmapset.title))
    //         .thumbnail(cover_url)
    //         .color(0xFF69B4) // Rose osu!
    //         .field("", format!("Mapped by **{}**", &beatmap.beatmapset.creator), false)
    //         .field("", format!("OD **`{:.1}`** • Duration **`{}`**\nBPM **`{:.0}`** • SR **`{:.2}`**", beatmap.accuracy, Self::format_duration(beatmap.hit_length), beatmap.bpm, beatmap.difficulty_rating), false)
    //         .field("Patterns detected", "See chart below", false)
    //         .url(&format!("https://osu.ppy.sh/beatmapsets/{}/#{}", beatmap.beatmapset_id, beatmap.id))
    //         .footer(CreateEmbedFooter::new("osu! Beatmap Analysis"))
    //         .timestamp(chrono::Utc::now())
    // }