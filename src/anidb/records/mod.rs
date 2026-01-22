use std::{
    error::Error,
    str::{FromStr, Split},
};

use anyhow::{bail, Context, Result};
use chrono::TimeZone;

mod anime;
mod episode;
mod file;
mod group;

pub use anime::Anime;
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

    fn take_str(&mut self) -> Result<&'a str> {
        // doesn't unescape
        self.input.next().context("Unexpected end of record")
    }

    pub fn take_string(&mut self) -> Result<String> {
        Ok(unescape(self.take_str()?))
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
        for item in self
            .take_str()?
            .split(sep)
            .filter(|s| !s.is_empty())
            .map(unescape)
        {
            result.push(item.parse().context("Failed to parse record")?);
        }
        Ok(result)
    }
}

fn unescape(s: &str) -> String {
    s.replace('`', "'").replace("<br />", "\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        #![allow(clippy::bool_assert_comparison)]
        #![deny(unused_variables)]

        let Anime { aid, dateflags, year, atype, related_aid_list, related_aid_type, romaji_name, kanji_name, english_name, short_name_list, episodes, special_ep_count, air_date, end_date, picname, nsfw, ann_id, characterid_list, specials_count, credits_count, other_count, trailer_count, parody_count } = Anime::parse("1|0|2000|TV|1'2|3'4|Don`t Toy With Me, Miss Nagatoro|イジらないで、長瀞さん|Don`t Toy With Me, Miss Nagatoro|Nagatoro'Don`tToyWithMe|12|0|86400|86400|nagatoro.jpg|0|0|1,2,3|0|0|0|0|0")
            .unwrap();

        assert_eq!(aid, 1);
        assert_eq!(dateflags, 0);
        assert_eq!(year, "2000");
        assert_eq!(atype, "TV");
        assert_eq!(related_aid_list, vec![1, 2]);
        assert_eq!(related_aid_type, vec!["3", "4"]);
        assert_eq!(romaji_name, "Don't Toy With Me, Miss Nagatoro");
        assert_eq!(kanji_name, "イジらないで、長瀞さん");
        assert_eq!(english_name, "Don't Toy With Me, Miss Nagatoro");
        assert_eq!(short_name_list, vec!["Nagatoro", "Don'tToyWithMe"]);
        assert_eq!(episodes, 12);
        assert_eq!(special_ep_count, 0);
        assert_eq!(air_date, chrono::Utc.with_ymd_and_hms(1970, 1, 2, 0, 0, 0).unwrap());
        assert_eq!(end_date, chrono::Utc.with_ymd_and_hms(1970, 1, 2, 0, 0, 0).unwrap());
        assert_eq!(picname, "nagatoro.jpg");
        assert_eq!(nsfw, false);
        assert_eq!(ann_id, 0);
        assert_eq!(characterid_list, vec![1, 2, 3]);
        assert_eq!(specials_count, 0);
        assert_eq!(credits_count, 0);
        assert_eq!(other_count, 0);
        assert_eq!(trailer_count, 0);
        assert_eq!(parody_count, 0);
    }
}
