use serde::Deserialize;

pub const ENDPOINT: &str = "https://status.animebytes.tv/api/status";

#[derive(Deserialize)]
pub struct StatusResponse {
    pub tracker: TrackerStatus,
    /// Could be maintenance
    #[serde(deserialize_with = "deserialize_struct")]
    pub site: Status,
    #[serde(deserialize_with = "deserialize_struct")]
    pub irc: Status,
    #[serde(deserialize_with = "deserialize_struct")]
    pub mei: Status,
}

#[derive(Deserialize)]
pub struct TrackerStatus {
    /// Could be partial outage
    #[serde(deserialize_with = "deserialize_status")]
    pub status: Status,
    pub details: Vec<TrackerDetails>,
}

#[derive(Deserialize)]
pub struct TrackerDetails {
    #[serde(deserialize_with = "deserialize_status")]
    pub status: Status,
    pub ip: String,
}

#[derive(Clone, Copy, Deserialize)]
pub enum Status {
    Offline,
    Online,
    /// Partial outage for trackers, maintenance for site
    MaintenanceOrPartialOutage,
}

fn deserialize_struct<'de, D>(deserializer: D) -> Result<Status, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Inner {
        #[serde(deserialize_with = "deserialize_status")]
        status: Status,
    }

    let inner = Inner::deserialize(deserializer)?;
    Ok(inner.status)
}

fn deserialize_status<'de, D>(deserializer: D) -> Result<Status, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = i32::deserialize(deserializer)?;
    match s {
        0 => Ok(Status::Offline),
        1 => Ok(Status::Online),
        2 => Ok(Status::MaintenanceOrPartialOutage),
        _ => Err(serde::de::Error::custom(format!("invalid status: {}", s))),
    }
}
