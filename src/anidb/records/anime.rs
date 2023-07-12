use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Record, RecordSplit};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Anime {
    pub aid: u32,
    pub dateflags: i32,
    pub year: String,
    pub atype: String,
    pub related_aid_list: Vec<u32>,
    pub related_aid_type: Vec<String>,
    pub romaji_name: String,
    pub kanji_name: String,
    pub english_name: String,
    pub short_name_list: Vec<String>,
    pub episodes: i32,
    pub special_ep_count: i32,
    pub air_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub picname: String,
    pub nsfw: bool,
    pub characterid_list: Vec<u32>,
    pub specials_count: i32,
    pub credits_count: i32,
    pub other_count: i32,
    pub trailer_count: i32,
    pub parody_count: i32,
}

impl Record for Anime {
    fn parse(input: &str) -> Result<Self> {
        let mut fields = RecordSplit::new(input);

        Ok(Self {
            aid: fields.take_parsed()?,
            dateflags: fields.take_parsed()?,
            year: fields.take_string()?,
            atype: fields.take_string()?,
            related_aid_list: fields.take_separated('\'')?,
            related_aid_type: fields.take_separated('\'')?,
            romaji_name: fields.take_string()?,
            kanji_name: fields.take_string()?,
            english_name: fields.take_string()?,
            short_name_list: fields.take_separated('\'')?,
            episodes: fields.take_parsed()?,
            special_ep_count: fields.take_parsed()?,
            air_date: fields.take_timestamp()?,
            end_date: fields.take_timestamp()?,
            picname: fields.take_string()?,
            nsfw: fields.take_bool()?,
            characterid_list: fields.take_separated(',')?,
            specials_count: fields.take_parsed()?,
            credits_count: fields.take_parsed()?,
            other_count: fields.take_parsed()?,
            trailer_count: fields.take_parsed()?,
            parody_count: fields.take_parsed()?,
        })
    }
}
