use std::time::{Duration, SystemTime};

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlRequest<T> {
    pub query: &'static str,
    pub variables: T,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQlResponse<T> {
    pub data: T,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub total: u64,
    pub per_page: u64,
    pub current_page: u64,
    pub last_page: u64,
    pub has_next_page: bool,
}

fn deserialize_systemtime<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
where
    D: Deserializer<'de>,
{
    let time = u64::deserialize(deserializer)?;
    Ok(SystemTime::UNIX_EPOCH + Duration::from_secs(time))
}

fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let time = u64::deserialize(deserializer)?;
    Ok(Duration::from_secs(time))
}
