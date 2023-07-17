pub mod data {
    pub mod airing;
    pub mod common;
}

use anyhow::Result;
use serde::Deserialize;

use self::data::{airing::Media, common::GraphQlRequest};
use crate::anichart::data::{
    airing::AiringParams,
    common::{GraphQlResponse, PageInfo},
};

const ENDPOINT: &str = "https://graphql.anilist.co/";

pub async fn airing() -> Result<Vec<Media>> {
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Inner {
        page: InnerPage,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct InnerPage {
        pub page_info: PageInfo,
        pub media: Vec<Media>,
    }

    Ok(reqwest::Client::new()
        .post(ENDPOINT)
        .json(&GraphQlRequest {
            query: include_str!("data/airing.graphql"),
            variables: AiringParams {
                page: 1,
                per_page: 20,
                season: "SUMMER",
                season_year: 2023,
            },
        })
        .send()
        .await?
        .json::<GraphQlResponse<Inner>>()
        .await?
        .data
        .page
        .media)
}
