use std::{
    error::Error,
    str::{FromStr, Split},
};

use anyhow::{bail, Context, Result};

mod anime;
mod episode;
mod file;
mod group;

pub use anime::Anime;
use chrono::TimeZone;
pub use episode::Episode;
pub use file::File;
pub use group::Group;

pub trait Record: Sized {
    fn parse(input: &str) -> Result<Self>;
}

struct RecordSplit<'a> {
    input: Split<'a, char>,
}

impl<'a> RecordSplit<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input: input.split('|') }
    }

    pub fn take_str(&mut self) -> Result<&'a str> {
        self.input.next().context("Unexpected end of record")
    }

    pub fn take_string(&mut self) -> Result<String> {
        Ok(self.take_str()?.to_string())
    }

    pub fn take_bool(&mut self) -> Result<bool> {
        match self.take_str()? {
            "1" => Ok(true),
            "0" => Ok(false),
            _ => bail!("Failed to parse bool"),
        }
    }

    pub fn take_parsed<T>(&mut self) -> Result<T>
    where
        T: FromStr,
        T::Err: Error + Send + Sync + 'static,
    {
        self.take_str()?.parse().context("Failed to parse record")
    }

    pub fn take_timestamp(&mut self) -> Result<chrono::DateTime<chrono::Utc>> {
        let timestamp = self.take_parsed().context("Failed to parse timestamp")?;

        chrono::Utc
            .timestamp_opt(timestamp, 0)
            .single()
            .context("Timestamp out of range")
    }

    pub fn take_separated<T>(&mut self, sep: char) -> Result<Vec<T>>
    where
        T: FromStr,
        T::Err: Error + Send + Sync + 'static,
    {
        let mut result = Vec::new();
        for item in self.take_str()?.split(sep).filter(|s| !s.is_empty()) {
            result.push(item.parse().context("Failed to parse record")?);
        }
        Ok(result)
    }
}
