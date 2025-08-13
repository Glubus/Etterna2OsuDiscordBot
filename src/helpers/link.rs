use regex::Regex;
use anyhow::{Result, anyhow};
use once_cell::sync::Lazy;

static LINK_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"osu\.ppy\.sh/beatmapsets/\d+#[^/]+/(\d+)").unwrap()
});

pub fn extract_beatmap_id(beatmapset_link: &str) -> Result<u32> {
    LINK_REGEX
        .captures(beatmapset_link)
        .and_then(|cap| cap.get(1).map(|m| m.as_str()))
        .and_then(|id| id.parse::<u32>().ok())
        .ok_or_else(|| anyhow!("Format de lien invalide ou ID non numérique"))
}

pub fn transform_link(beatmapset_link: &str) -> Result<(u32, String)> {
    let beatmap_id = extract_beatmap_id(beatmapset_link)?;
    let osu_link = format!("https://osu.ppy.sh/osu/{}", beatmap_id);
    Ok((beatmap_id, osu_link))
}