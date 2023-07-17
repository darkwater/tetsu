use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringParams {
    pub page: i32,
    pub per_page: i32,
    pub season: &'static str,
    pub season_year: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub id: i32,
    pub title: Title,
    pub next_airing_episode: NextAiringEpisode,
    pub cover_image: CoverImage,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub romaji: String,
    pub english: String,
    pub native: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextAiringEpisode {
    pub airing_at: i64,
    pub time_until_airing: i64,
    pub episode: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverImage {
    pub large: String,
}
