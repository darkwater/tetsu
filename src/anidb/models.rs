use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
    #[serde(with = "chrono::serde::ts_seconds")]
    pub aired_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
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
    #[serde(with = "chrono::serde::ts_seconds")]
    pub air_date: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
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
    #[serde(with = "chrono::serde::ts_seconds")]
    pub aired: DateTime<Utc>,
    pub etype: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub gid: u32,
    pub rating: i32,
    pub votes: i32,
    pub acount: i32,
    pub fcount: i32,
    pub name: String,
    pub short: String,
    pub irc_channel: String,
    pub irc_server: String,
    pub url: String,
    pub picname: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub foundeddate: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub disbandeddate: DateTime<Utc>,
    pub dateflags: i16,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub lastreleasedate: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub lastactivitydate: DateTime<Utc>,
    pub grouprelations: String,
}
