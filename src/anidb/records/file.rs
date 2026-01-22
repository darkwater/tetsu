use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Record, RecordSplit};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub fid: u32,
    pub aid: u32,
    pub eid: u32,
    pub gid: u32,
    pub state: i16,
    pub size: i64,
    pub ed2k: String,
    pub colour_depth: String,
    pub quality: String,
    pub source: String,
    pub audio_codec_list: Vec<String>,
    pub audio_bitrate_list: Vec<i32>,
    pub video_codec: Vec<String>,
    pub video_bitrate: Vec<String>,
    pub video_resolution: Vec<String>,
    pub dub_language: String,
    pub sub_language: String,
    pub length_in_seconds: i32,
    pub description: String,
    pub aired_date: DateTime<Utc>,
}

impl Record for File {
    fn parse(input: &str) -> Result<Self> {
        let mut fields = RecordSplit::new(input);

        Ok(Self {
            fid: fields.take_parsed()?,
            aid: fields.take_parsed()?,
            eid: fields.take_parsed()?,
            gid: fields.take_parsed()?,
            state: fields.take_parsed()?,
            size: fields.take_parsed()?,
            ed2k: fields.take_string()?,
            colour_depth: fields.take_string()?,
            quality: fields.take_string()?,
            source: fields.take_string()?,
            audio_codec_list: fields.take_separated('\'')?,
            audio_bitrate_list: fields.take_separated('\'')?,
            video_codec: fields.take_separated('\'')?,
            video_bitrate: fields.take_separated('\'')?,
            video_resolution: fields.take_separated('\'')?,
            dub_language: fields.take_string()?,
            sub_language: fields.take_string()?,
            length_in_seconds: fields.take_parsed()?,
            description: fields.take_string()?,
            aired_date: fields.take_timestamp()?,
        })
    }
}
