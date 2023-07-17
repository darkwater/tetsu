use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use serde::{Deserialize, Deserializer};

pub const ENDPOINT: &str = "https://animebytes.tv/api/stats/{passkey}";
pub const COMPAT_VERSION: i32 = 2;

#[derive(Deserialize)]
pub struct SiteStatsResponse {
    pub git: String,
    pub api: ApiInfo,
    pub freeleech: FreeleechInfo,
    pub stats: Stats,
}

#[derive(Deserialize)]
pub struct ApiInfo {
    /// Version of the API
    pub version: String,
    /// Compatibility version of the API, incremented when backwards incompatible changes are made
    pub compat: i32,
}

#[derive(Deserialize)]
pub struct FreeleechInfo {
    /// When the current sitewide freeleech ends
    #[serde(deserialize_with = "deserialize_systemtime")]
    pub sitewide: Option<SystemTime>,
    /// When the current personal freeleech ends
    #[serde(deserialize_with = "deserialize_systemtime")]
    pub personal: Option<SystemTime>,
}

#[derive(Deserialize)]
pub struct Stats {
    pub site: SiteStats,
    pub personal: PersonalStats,
}

#[derive(Deserialize)]
pub struct SiteStats {
    pub torrents: HashMap<String, i32>,
    pub peers: HashMap<String, i32>,
    pub users: HashMap<String, i32>,
    pub requests: HashMap<String, i32>,
    pub classes: HashMap<String, i32>,
    pub forums: HashMap<String, i32>,
}

#[derive(Deserialize)]
pub struct PersonalStats {
    pub yen: YenStats,
    pub hnrs: HnrStats,
    pub upload: UploadStats,
    pub download: DownloadStats,
    pub torrents: TorrentStats,
    /// Number of people the user invited
    pub invited: i32,
    pub forums: ForumStats,
    /// Number of user comments on the user's profile
    pub pcomments: i32,
    /// User class name
    pub class: String,
}

#[derive(Deserialize)]
pub struct YenStats {
    /// Yen generation per day based on hourly projection
    pub day: i32,
    /// Yen generation per hour based on current 15 minute interval projection
    pub hour: i32,
    /// Amount of yen that user has
    pub now: i32,
}

#[derive(Deserialize)]
pub struct HnrStats {
    /// Current potential hit and runs for user
    pub potential: i32,
    /// Current active hit and runs for user
    pub active: i32,
}

#[derive(Deserialize)]
pub struct UploadStats {
    /// Raw upload for user in bytes
    pub raw: i64,
    /// Accountable upload for user in bytes
    pub accountable: i64,
}

#[derive(Deserialize)]
pub struct DownloadStats {
    /// Raw download for user in bytes
    pub raw: i64,
    /// Accountable download for user in bytes
    pub accountable: i64,
}

#[derive(Deserialize)]
pub struct TorrentStats {
    /// Amount of torrents user is seeding
    pub seeding: i32,
    /// Amount of torrents user is leeching
    pub leeching: i32,
    /// Amount of torrents user has snatched
    pub snatched: i32,
    /// Amount of torrents user has uploaded
    pub uploaded: i32,
    /// Amount of torrents user has uploaded and been pruned
    pub pruned: i32,
    /// Total seed size in bytes
    #[serde(rename = "ssize")]
    pub seed_size: i64,
    /// Total seed time
    #[serde(deserialize_with = "deserialize_duration")]
    #[serde(rename = "sttime")]
    pub total_seed_time: Duration,
    /// Average seed time
    #[serde(deserialize_with = "deserialize_duration")]
    #[serde(rename = "satime")]
    pub avg_seed_time: Duration,
}

#[derive(Deserialize)]
pub struct ForumStats {
    /// Number of accountable posts (excluding spam forums and deleted posts)
    pub posts: i32,
    /// Number of forum threads the user started
    pub topics: i32,
}

fn deserialize_systemtime<'de, D>(deserializer: D) -> Result<Option<SystemTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let time = u64::deserialize(deserializer)?;
    if time == 0 {
        Ok(None)
    } else {
        Ok(Some(SystemTime::UNIX_EPOCH + Duration::from_secs(time)))
    }
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let time = u64::deserialize(deserializer)?;
    Ok(Duration::from_secs(time))
}
