use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScrapeResponse {
    /// Total number of matched groups
    results: usize,
    pagination: Pagination,
    groups: Vec<AnimeGroup>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Pagination {
    /// Current page
    current: usize,
    /// Total number of pages
    max: usize,
    limit: PaginationLimit,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaginationLimit {
    /// Minimum number of groups per page
    min: usize,
    /// Current number of groups per page
    coerced: usize,
    /// Maximum number of groups per page
    max: usize,
}

// // Groups for type = 'anime'
// [
//     {
//       "ID": (int) ID of group [50572],
//       "CategoryName": (string) category of group [Light Novel],
//       "FullName": (string) full HTML encoded group name [Cooking with Wild Game - Light Novel  [2015]],
//       "GroupName": (string) group name [Light Novel],
//       "SeriesID": (int) ID of series the group belongs to [49712]
//       "SeriesName": (string) name of the series the group belongs to [Cooking with Wild Game]
//       "Year": (string) group's year [2015],
//       "Image": (string) group's cover image [https://mei.animebytes.tv/YSNbsYanyDC.jpg],
//       "SynonymnsV2": (object) synonyms, indexed by their type,
//       "Snatched": (int) amount of snatches [67],
//       "Comments": (int) amount of comments [1],
//       "Links": (object) list of links,
//       "Votes": (int) amount of votes [0],
//       "AvgVote": (int) average vote [0],
//       "Description": (string|null) BBcode representation of description,
//       "DescriptionHTML": (string) HTML encoded representation of description,
//       "EpCount": (int) amount of episodes or volumes for group [18],
//       "StudioList": (string|null) internal format, list of studios/publishers [Hobby Japan///1249|J-Novel Club///3081],
//       "PastWeek": (int) amount of torrents added in past week [0],
//       "Incomplete": (bool) true if there are no complete torrents for this group,
//       "Ongoing": (bool) is the group airing (applicable only for anime),
//       "Tags": (array of strings) list of tags
//       "Torrents": (array of objects) list of torrents
//     }
// ]

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimeGroup {
    /// ID of group
    #[serde(rename = "ID")]
    id: usize,
    /// Category of group
    category_name: String,
    /// Full HTML encoded group name
    full_name: String,
    /// Group name
    group_name: String,
    /// ID of series the group belongs to
    series_id: usize,
    /// Name of the series the group belongs to
    series_name: String,
    /// Group's year
    year: String,
    /// Group's cover image
    image: String,
    /// Synonyms, indexed by their type
    synonymns_v2: Option<SynonymnsV2>,
    /// Amount of snatches
    snatched: usize,
    /// Amount of comments
    comments: usize,
    /// List of links
    links: HashMap<String, String>,
    /// Amount of votes
    votes: usize,
    /// Average vote
    avg_vote: usize,
    /// BBcode representation of description
    description: Option<String>,
    /// HTML encoded representation of description
    description_html: String,
    /// Amount of episodes or volumes for group
    ep_count: usize,
    /// Internal format, list of studios/publishers
    /// eg. "Hobby Japan///1249|J-Novel Club///3081"
    studio_list: Option<String>,
    /// Amount of torrents added in past week
    past_week: usize,
    /// True if there are no complete torrents for this group
    incomplete: bool,
    /// Is the group airing (applicable only for anime)
    ongoing: bool,
    /// List of tags
    tags: Vec<String>,
    /// List of torrents
    torrents: Vec<Torrent>,
}

// // SynonymnsV2
// {
// 	"Japanese": (string) japanese title, in kanji [異世界召喚は二度目です],
// 	"Romaji": (string) japanese title, transcribed to romaji [Isekai Shoukan wa Nidome desu],
// 	"Alternative": (string) alternative title, separated with comma if multiple [Isenido]
// }

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SynonymnsV2 {
    /// Japanese title, in kanji
    japanese: String,
    /// Japanese title, transcribed to romaji
    romaji: String,
    /// Alternative title, separated with comma if multiple
    alternative: String,
}

// // Torrents
// [
//         {
//           "ID": (int) ID of torrent [438333],
//           "EditionData": (object) edition data for torrent,
//           "RawDownMultiplier": (int) download multiplier applied to torrent [0],
//           "RawUpMultiplier": (int) upload multiplier applied to torrent [1],
//           "Link": (string) download link for torrent [https://animebytes.tv/torrent/438333/download/{:passkey}],
//           "Property": (string) property string of all torrent's metadata [Blu-ray | MKV | h264 | 1080p | FLAC 2.0 | Softsubs (PhyStein) | Freeleech],
//           "Snatched": (int) number of snatches [6],
//           "Seeders": (int) number of seeders [23],
//           "Leechers": (int) number of leechers [1],
//           "Status": (int) torrent state, either 0 (visible) or 1 (pruned) [0],
//           "Size": (int) size of torrents in bytes [17304126999],
//           "FileCount": (int) number of files in torrent [1],
//           "FileList": (object) list of files and their sizes,
//           "UploadTime": (sqltime) datetime the torrent was uploaded [2020-02-25 08:59:54]
//         }
// ]

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Torrent {
    /// ID of torrent
    #[serde(rename = "ID")]
    id: usize,
    /// Edition data for torrent
    edition_data: Option<EditionData>,
    /// Download multiplier applied to torrent
    raw_down_multiplier: usize,
    /// Upload multiplier applied to torrent
    raw_up_multiplier: usize,
    /// Download link for torrent
    link: String,
    /// Property string of all torrent's metadata
    property: String,
    /// Number of snatches
    snatched: usize,
    /// Number of seeders
    seeders: usize,
    /// Number of leechers
    leechers: usize,
    /// Torrent state, either 0 (visible) or 1 (pruned)
    status: usize,
    /// Size of torrents in bytes
    size: usize,
    /// Number of files in torrent
    file_count: usize,
    /// List of files and their sizes
    file_list: Vec<GroupFile>,
    /// Datetime the torrent was uploaded
    upload_time: String,
}

// // EditionData
// {
// 	"EditionTitle": (string) name of the edition
// }

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EditionData {
    /// Name of the edition
    edition_title: String,
}

// // FileList
// {
// 	"filename": (string) UTF-8 filename
//         "size": (int) size in bytes
// }

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GroupFile {
    /// UTF-8 filename
    filename: String,
    /// Size in bytes
    size: usize,
}
