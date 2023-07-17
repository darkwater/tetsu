use anyhow::Result;
use serde::Deserialize;

use self::data::{site_stats::SiteStatsResponse, status::StatusResponse};

pub mod data {
    pub mod site_stats;
    pub mod status;
    pub mod scrape {
        pub const ENDPOINT: &str = "https://animebytes.tv/scrape.php";
        pub mod params;
        pub mod response;
    }
}

pub async fn site_stats() -> Result<SiteStatsResponse> {
    let url = data::site_stats::ENDPOINT.replace("{passkey}", env!("ANIMEBYTES_PASS"));
    Ok(reqwest::get(url).await?.json().await?)
}

pub async fn status() -> Result<StatusResponse> {
    #[derive(Deserialize)]
    pub struct ActualResponse {
        pub status: StatusResponse,
    }

    let url = data::status::ENDPOINT;
    Ok(reqwest::get(url)
        .await?
        .json::<ActualResponse>()
        .await?
        .status)
}
