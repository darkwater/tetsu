use std::{
    fs,
    io::Write,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

use crate::{
    anidb::records::{Anime, Episode, File, Group},
    ANIDB,
};

#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub anime: Option<Anime>,
    pub episode: Option<Episode>,
    pub group: Option<Group>,
    pub file: File,
}

pub async fn write(folder: &Path, playlist: &Path) -> anyhow::Result<()> {
    let mut dirs = vec![folder.to_owned()];
    let mut files = vec![];

    while let Some(dir) = dirs.pop() {
        let rd = fs::read_dir(dir).unwrap();
        for entry in rd.map_while(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            } else {
                files.push(path);
            }
        }
    }

    let mut info = vec![];
    let base = playlist.parent().map(Path::to_path_buf).unwrap_or_default();

    for path in files {
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

        let anime = anidb.anime_by_aid(file.aid).await?;
        let episode = anidb.episode_by_eid(file.eid).await?;
        let group = anidb.group_by_gid(file.gid).await?;

        let path = path.strip_prefix(&base).unwrap_or(&path).to_owned();

        info.push(FileInfo { path, anime, episode, group, file });
    }

    if info.is_empty() {
        anyhow::bail!("No indexed files found in the specified folder");
    }

    info.sort_by(|a, b| {
        let epno_order = a
            .episode
            .as_ref()
            .map(|e| &e.epno)
            .cmp(&b.episode.as_ref().map(|e| &e.epno));

        let path_order = a.path.cmp(&b.path);

        epno_order.then(path_order)
    });

    let mut file = fs::File::create(playlist)?;
    writeln!(file, "#EXTM3U")?;

    if let Some(Anime { aid, romaji_name, .. }) = info.iter().find_map(|i| i.anime.as_ref()) {
        writeln!(file, "#PLAYLIST:{romaji_name}")?;
        writeln!(file, "#EXT-ANIDB-AID:{aid}")?;
    }

    if let Some(Group { gid, name, .. }) = info.iter().find_map(|i| i.group.as_ref()) {
        writeln!(file, "#EXT-ANIDB-GID:{gid}")?;
        writeln!(file, "#EXT-ANIDB-GROUP:{name}")?;
    }

    for info in info {
        if let Some(File { fid, .. }) = Some(&info.file) {
            writeln!(file, "#EXT-ANIDB-FID:{fid}")?;
        }

        if let Some(Episode { eid, epno, romaji, .. }) = info.episode.as_ref() {
            writeln!(file, "#EXT-ANIDB-EID:{eid}")?;

            let length = info.file.length_in_seconds;
            writeln!(file, "#EXTINF:{length},{epno}. {romaji}")?;
        }
        file.write_all(info.path.as_os_str().as_bytes())?;
        writeln!(file)?;
    }

    Ok(())
}
