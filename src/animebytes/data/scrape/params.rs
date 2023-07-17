use std::collections::HashMap;

use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Serialize)]
pub struct ScrapeParams {
    torrent_pass: String,
    username: String,
    #[serde(flatten)]
    general: GeneralParams,
    #[serde(flatten)]
    r#type: ScrapeType,
}

#[derive(Default, Serialize)]
pub struct GeneralParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<SortKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    way: Option<SortDir>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_bool")]
    freeleech: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_bool")]
    showhidden: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortKey {
    Relevance,
    Grouptime,
    Name,
    Year,
    Size,
    Rating,
    Votes,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDir {
    Asc,
    Desc,
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ScrapeType {
    Music(MusicParams),
    Anime(AnimeParams),
}

#[derive(Default, Serialize)]
#[serde(default)]
pub struct MusicParams {
    #[serde(skip_serializing_if = "String::is_empty")]
    artistnames: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    groupname: String,
    #[serde(flatten)]
    other: HashMap<String, String>,
}

#[derive(Default, Serialize)]
#[serde(default)]
pub struct AnimeParams {
    searchstr: String,
    search_type: AnimeSearchType,
    hentai: HentaiFilter,
    #[serde(flatten)]
    other: HashMap<String, String>,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AnimeSearchType {
    #[default]
    Title,
    People,
    Company,
}

#[derive(Default, Serialize_repr)]
#[repr(u8)]
pub enum HentaiFilter {
    Exclude = 0,
    Fap = 1,
    #[default]
    Show = 2,
    Uncensored = 3,
}

fn serialize_bool<S>(b: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(b) = b {
        serializer.serialize_i32(if *b { 1 } else { 0 })
    } else {
        serializer.serialize_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrape_params_format() {
        let from = ScrapeParams {
            torrent_pass: "foobar".into(),
            username: "john".into(),
            general: GeneralParams {
                sort: Some(SortKey::Name),
                ..Default::default()
            },
            r#type: ScrapeType::Anime(AnimeParams {
                searchstr: "foo".into(),
                other: [("anime[foo]".into(), "1".into())]
                    .iter()
                    .cloned()
                    .collect(),
                ..Default::default()
            }),
        };

        // note that we don't actually send json to animebytes (can we?)
        // but this is just to test the structure of the parameters
        let to = serde_json::json!({
            "torrent_pass": "foobar",
            "username": "john",
            "sort": "name",
            "type": "anime",
            "searchstr": "foo",
            "search_type": "title",
            "hentai": 2,
            "anime[foo]": "1",
        });

        assert_eq!(serde_json::to_value(from).unwrap(), to);
    }
}
