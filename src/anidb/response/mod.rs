use std::str::FromStr;

use anyhow::Context;
use num_traits::FromPrimitive;

use self::codes::ResponseCode;
use super::records::Record;

pub mod codes;

pub struct Response {
    pub code: ResponseCode,
    pub message: String,
    pub records: Vec<String>,
}

impl FromStr for Response {
    type Err = anyhow::Error;

    // 220 MESSAGE
    // 1|2|foo|3|bar
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        let mut lines = _s.lines();

        let first_line = lines.next().context("Empty response")?;
        let (code, message) = first_line.split_once(' ').context("Invalid response")?;

        let message = message.trim().to_string();
        let code = ResponseCode::from_u32(code.parse().context("Invalid response code")?)
            .context("Unknown response code")?;

        let records = lines.map(|line| line.to_string()).collect();

        Ok(Response { code, message, records })
    }
}

impl Response {
    /// For messages such as 200 xxxx LOGIN ACCEPTED, return the xxxx part
    pub fn data(&self) -> Option<&str> {
        self.message.split_once(' ').map(|p| p.0)
    }

    pub fn records_as<T: Record>(&self) -> impl Iterator<Item = anyhow::Result<T>> + '_ {
        self.records.iter().map(|record| {
            T::parse(record).context(format!(
                "Failed to parse record as {}: {}",
                std::any::type_name::<T>(),
                record
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{Response, ResponseCode};

    // use serde_repr::Deserialize_repr;

    #[test]
    fn from_str() {
        let s = r#"220 FILE
fid|aid|eid|gid|state|size|ed2k|anidbfilename
fid|aid|eid|gid|state|size|ed2k|anidbfilename"#;

        let response = Response::from_str(s).unwrap();

        assert_eq!(response.code, ResponseCode::File);
        assert_eq!(response.message, "FILE");
        assert_eq!(response.records.len(), 2);
    }

    #[test]
    fn with_data() {
        let s = r#"200 abcdef LOGIN ACCEPTED"#;

        let response = Response::from_str(s).unwrap();

        assert_eq!(response.code, ResponseCode::LoginAccepted);
        assert_eq!(response.message, "abcdef LOGIN ACCEPTED");
        assert_eq!(response.records.len(), 0);
        assert_eq!(response.data(), Some("abcdef"));
    }

    //     #[test]
    //     fn records_as() {
    //         #[derive(Deserialize)]
    //         struct Episode {
    //             pub eid: u32,
    //             pub aid: u32,
    //             pub length: i32,
    //             pub rating: i32,
    //             pub votes: i32,
    //             pub epno: String,
    //             pub eng: String,
    //             pub romaji: String,
    //             pub kanji: String,
    //             #[serde(with = "chrono::serde::ts_seconds")]
    //             pub aired: DateTime<Utc>,
    //             pub etype: EpisodeType,
    //         }

    //         #[derive(Deserialize_repr, Debug, PartialEq)]
    //         #[repr(i32)]
    //         enum EpisodeType {
    //             Regular = 1,
    //             Other = 6,
    //             Credit = 3,
    //             Trailer = 4,
    //             Special = 2,
    //             Parody = 5,
    //         }

    //         let s = r#"240 EPISODE
    // 1|2|3|-4|5|C01|Start|Hajimete|はじめて|946684800|3"#;

    //         let response = Response::from_str(s).unwrap();

    //         let episode = response.records_as::<Episode>().next().unwrap().unwrap();

    //         assert_eq!(episode.eid, 1);
    //         assert_eq!(episode.aid, 2);
    //         assert_eq!(episode.length, 3);
    //         assert_eq!(episode.rating, -4);
    //         assert_eq!(episode.votes, 5);
    //         assert_eq!(episode.epno, "C01");
    //         assert_eq!(episode.eng, "Start");
    //         assert_eq!(episode.romaji, "Hajimete");
    //         assert_eq!(episode.kanji, "はじめて");
    //         assert_eq!(
    //             episode.aired,
    //             DateTime::<Utc>::from_utc(
    //                 NaiveDate::from_ymd_opt(2000, 1, 1)
    //                     .unwrap()
    //                     .and_hms_opt(0, 0, 0)
    //                     .unwrap(),
    //                 Utc
    //             )
    //         );
    //         assert_eq!(episode.etype, EpisodeType::Credit);
    //     }
}
