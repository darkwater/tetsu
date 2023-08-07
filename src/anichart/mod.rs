use anyhow::Result;
use serde::Deserialize;

use self::data::common::GraphQlRequest;
use crate::anichart::data::{
    airing::AiringParams,
    by_mal_id::ByMalIdParams,
    common::{GraphQlResponse, PageInfo},
};

pub mod data {
    pub mod airing;
    pub mod by_mal_id;
    pub mod common;
}

pub mod linker;

const ENDPOINT: &str = "https://graphql.anilist.co/";

pub async fn airing() -> Result<Vec<data::airing::Media>> {
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Inner {
        page: InnerPage,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct InnerPage {
        pub page_info: PageInfo,
        pub media: Vec<data::airing::Media>,
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

pub async fn by_mal_id(mal_id: i32) -> Result<data::by_mal_id::Media> {
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Inner {
        media: data::by_mal_id::Media,
    }

    Ok(reqwest::Client::new()
        .post(ENDPOINT)
        .json(&GraphQlRequest {
            query: include_str!("data/by_mal_id.graphql"),
            variables: ByMalIdParams { mal_id },
        })
        .send()
        .await?
        .json::<GraphQlResponse<Inner>>()
        .await?
        .data
        .media)
}
