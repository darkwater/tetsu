use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Record, RecordSplit};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
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
    pub foundeddate: DateTime<Utc>,
    pub disbandeddate: DateTime<Utc>,
    pub dateflags: i16,
    pub lastreleasedate: DateTime<Utc>,
    pub lastactivitydate: DateTime<Utc>,
    pub grouprelations: String,
}

impl Record for Group {
    fn parse(input: &str) -> Result<Self> {
        let mut fields = RecordSplit::new(input);

        Ok(Self {
            gid: fields.take_parsed()?,
            rating: fields.take_parsed()?,
            votes: fields.take_parsed()?,
            acount: fields.take_parsed()?,
            fcount: fields.take_parsed()?,
            name: fields.take_string()?,
            short: fields.take_string()?,
            irc_channel: fields.take_string()?,
            irc_server: fields.take_string()?,
            url: fields.take_string()?,
            picname: fields.take_string()?,
            foundeddate: fields.take_timestamp()?,
            disbandeddate: fields.take_timestamp()?,
            dateflags: fields.take_parsed()?,
            lastreleasedate: fields.take_timestamp()?,
            lastactivitydate: fields.take_timestamp()?,
            grouprelations: fields.take_string()?,
        })
    }
}
