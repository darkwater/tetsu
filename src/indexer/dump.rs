use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context as _;
use serde::{Deserialize, Serialize};

use crate::{
    anidb::records::{Anime, Episode, File, Group},
    ANIDB,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDump {
    pub files: HashMap<PathBuf, File>,
    pub anime: Vec<Anime>,
    pub episodes: Vec<Episode>,
    pub groups: Vec<Group>,
}

pub async fn dump_json(folder: &Path, dump_path: &Path) -> anyhow::Result<()> {
    let mut dirs = vec![folder.to_owned()];
    let mut paths = vec![];

    while let Some(dir) = dirs.pop() {
        let rd = fs::read_dir(dir).unwrap();
        for entry in rd.map_while(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            } else {
                paths.push(path);
            }
        }
    }

    let base = dump_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_default();

    let mut aids = HashSet::new();
    let mut eids = HashSet::new();
    let mut gids = HashSet::new();
    let mut files = HashMap::new();

    for path in paths {
        let path_str = path.to_string_lossy();

        let Some(fid) = sqlx::query!("SELECT fid FROM indexed_files WHERE path = $1", path_str)
            .fetch_optional(crate::DB.get().await)
            .await?
            .and_then(|r| r.fid)
            .and_then(|s| s.try_into().ok())
        else {
            eprintln!("File can't be identified: {}", path_str);
            continue;
        };

        let mut anidb = ANIDB.write().await;

        let Some(file) = anidb.file_by_fid(fid).await? else {
            continue;
        };

        aids.insert(file.aid);
        eids.insert(file.eid);
        gids.insert(file.gid);

        let path = path.strip_prefix(&base).unwrap_or(&path).to_owned();
        let path = path.strip_prefix(".").unwrap_or(&path).to_owned();
        files.insert(path, file);
    }

    if files.is_empty() {
        anyhow::bail!("No indexed files found in the specified folder");
    }

    let mut anime = vec![];
    for aid in aids {
        let mut anidb = ANIDB.write().await;
        if let Some(a) = anidb.anime_by_aid(aid).await? {
            anime.push(a);
        } else {
            eprintln!("Anime not found for AID: {}", aid);
        }
    }

    let mut episodes = vec![];
    for eid in eids {
        let mut anidb = ANIDB.write().await;
        if let Some(e) = anidb.episode_by_eid(eid).await? {
            episodes.push(e);
        } else {
            eprintln!("Episode not found for EID: {}", eid);
        }
    }

    let mut groups = vec![];
    for gid in gids {
        let mut anidb = ANIDB.write().await;
        if let Some(g) = anidb.group_by_gid(gid).await? {
            groups.push(g);
        } else {
            eprintln!("Group not found for GID: {}", gid);
        }
    }

    let dump = DataDump { files, anime, episodes, groups };

    let mut file = fs::File::create(dump_path)?;
    serde_json::to_writer_pretty(&mut file, &dump).context("Failed to write JSON dump")?;

    Ok(())
}
