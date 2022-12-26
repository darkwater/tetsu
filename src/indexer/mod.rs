use std::{
    path::{Path, PathBuf},
    process::Termination,
    time::Duration,
};

use anyhow::{Context, Result};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tokio::{fs, sync::mpsc};

use crate::ANIDB;

mod ed2k;

#[derive(Debug)]
struct AnidbRequestHandoff {
    hash: String,
    file_path: PathBuf,
    pb: ProgressBar,
}

pub async fn index(path: &Path) -> Result<()> {
    let mpb = MultiProgress::new();
    crate::PROGRESS_BAR.write().unwrap().replace(mpb.clone());

    let (tx, rx) = mpsc::unbounded_channel();
    let anidb_task_handle = tokio::spawn(get_anidb_data_task(rx));

    let overall = mpb.add(ProgressBar::new(0));
    // overall.enable_steady_tick(Duration::from_millis(125));
    overall.set_style(
        ProgressStyle::default_bar()
            .progress_chars("== ")
            .template("[{elapsed_precise}] [{bar:32.cyan/blue}] {pos:.green}/{len:.blue} ({eta:.yellow}) {wide_msg}")
            .unwrap(),
    );
    overall.set_message("Building file list...");

    let mut dirs = vec![path.to_owned()];
    let mut files = vec![];

    while let Some(dir) = dirs.pop() {
        let mut rd = fs::read_dir(dir).await.unwrap();
        while let Some(entry) = rd.next_entry().await.unwrap() {
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path);
            } else {
                files.push(path);
                overall.inc_length(1);
            }
        }
    }

    overall.reset_eta();
    overall.set_message("Indexing files...");

    for file_path in files {
        let utf_path = file_path.to_string_lossy();
        let in_db = sqlx::query!("SELECT path FROM indexed_files WHERE path = ?", utf_path)
            .fetch_optional(crate::DB.get().await)
            .await?
            .is_some();

        if in_db {
            overall.inc(1);
            continue;
        }

        let pb = mpb.insert_before(&overall, ProgressBar::new(0));
        pb.set_style(
            ProgressStyle::default_bar()
                .progress_chars("== ")
                .template("[{elapsed_precise}] [{bar:32.cyan/blue}] {spinner:.green} {wide_msg}")
                .unwrap(),
        );

        pb.set_message(
            file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
        );

        pb.enable_steady_tick(Duration::from_millis(125));

        let hash = tokio::task::block_in_place(|| ed2k::hash_file(&file_path, &pb).unwrap());

        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {spinner:.green} {wide_msg}")
                .unwrap(),
        );

        tx.send(AnidbRequestHandoff { hash, file_path, pb })
            .unwrap();

        overall.inc(1);
    }

    overall.set_message("Waiting for AniDB data...");

    drop(tx);
    anidb_task_handle.await.unwrap();

    overall.finish_with_message("Done!");

    crate::PROGRESS_BAR.write().unwrap().take();

    Ok(())
}

async fn get_anidb_data_task(mut rx: mpsc::UnboundedReceiver<AnidbRequestHandoff>) {
    let mut errors = 0u32;

    while let Some(handoff) = rx.recv().await {
        match get_anidb_data(handoff).await {
            Ok(()) => errors = errors.saturating_sub(1),
            res @ Err(_) => {
                if errors >= 5 {
                    log::error!("Too many errors, aborting.");
                    break;
                }

                // errors += 1;
                // log::error!("Error: {e}");

                Termination::report(res);
                std::process::exit(1);
            }
        }
    }
}

async fn get_anidb_data(
    AnidbRequestHandoff { hash, file_path, pb }: AnidbRequestHandoff,
) -> Result<()> {
    let size = file_path.metadata().unwrap().len() as i64;

    let mut anidb = ANIDB.write().await;

    let anidb_file = anidb
        .file_by_ed2k(size, &hash)
        .await
        .context("Failed to get file data from AniDB")?;

    if let Some(ref anidb_file) = anidb_file {
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {spinner:.green} {wide_msg:.green}")
                .unwrap(),
        );

        if let Some(anime) = anidb
            .anime_by_aid(anidb_file.aid)
            .await
            .context("Failed to get anime data from AniDB")?
        {
            pb.set_message(anime.kanji_name.clone());

            if let Some(episode) = anidb
                .episode_by_eid(anidb_file.eid)
                .await
                .context("Failed to get episode data from AniDB")?
            {
                pb.set_message(format!("{} - {}", anime.kanji_name.clone(), episode.kanji));

                if let Some(group) = anidb
                    .group_by_gid(anidb_file.gid)
                    .await
                    .context("Failed to get group data from AniDB")?
                {
                    pb.set_message(format!(
                        "{} - {} [{}]",
                        anime.kanji_name.clone(),
                        episode.kanji,
                        group.name
                    ));
                }
            }
        }
    } else {
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {spinner:.green} {wide_msg:.yellow}")
                .unwrap(),
        );

        pb.set_message(format!("Not found: {}", file_path.display()));
    }

    drop(anidb);

    let utf_path = file_path.to_string_lossy();
    let utf_name = file_path.file_name().unwrap().to_string_lossy();
    let now = chrono::Utc::now().timestamp();
    let fid = anidb_file.map(|f| f.fid);

    sqlx::query!(
            "INSERT INTO indexed_files (path, filename, filesize, ed2k, fid, first_seen, last_updated) VALUES (?, ?, ?, ?, ?, ?, ?)",
            utf_path,
            utf_name,
            size,
            hash,
            fid,
            now,
            now,
        )
        .execute(crate::DB.get().await)
        .await?;

    pb.finish();

    Ok(())
}
