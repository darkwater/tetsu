use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Record, RecordSplit};

#[derive(Serialize, Deserialize)]
pub struct Episode {
    pub eid: u32,
    pub aid: u32,
    pub length: i32,
    pub rating: i32,
    pub votes: i32,
    pub epno: String,
    pub eng: String,
    pub romaji: String,
    pub kanji: String,
    pub aired: DateTime<Utc>,
    pub etype: i32,
}

impl Record for Episode {
    fn parse(input: &str) -> Result<Self> {
        let mut fields = RecordSplit::new(input);

        Ok(Self {
            eid: fields.take_parsed()?,
            aid: fields.take_parsed()?,
            length: fields.take_parsed()?,
            rating: fields.take_parsed()?,
            votes: fields.take_parsed()?,
            epno: fields.take_string()?,
            eng: fields.take_string()?,
            romaji: fields.take_string()?,
            kanji: fields.take_string()?,
            aired: fields.take_timestamp()?,
            etype: fields.take_parsed()?,
        })
    }
}
