use std::{path::Path, time::Duration};

use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tokio::fs;

use crate::ANIDB;

mod ed2k;

pub async fn index(path: &Path) -> Result<()> {
    let mpb = MultiProgress::new();
    crate::PROGRESS_BAR.write().unwrap().replace(mpb.clone());

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
        let pb = mpb.insert_before(&overall, ProgressBar::new(0));
        pb.enable_steady_tick(Duration::from_millis(125));
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

        let utf_path = file_path.to_string_lossy();
        let in_db = sqlx::query!("SELECT path FROM indexed_files WHERE path = ?", utf_path)
            .fetch_optional(crate::DB.get().await)
            .await?
            .is_some();

        if in_db {
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {spinner:.green} {wide_msg:.blue}")
                    .unwrap(),
            );

            pb.set_message(format!("Already indexed: {}", file_path.display()));
            overall.inc(1);
            pb.finish();
            continue;
        }

        let hash = tokio::task::block_in_place(|| ed2k::hash_file(&file_path, &pb).unwrap());

        let size = file_path.metadata().unwrap().len() as i64;

        let anidb_file = ANIDB.write().await.file_by_ed2k(size, &hash).await?;

        if let Some(ref anidb_file) = anidb_file {
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {spinner:.green} {wide_msg:.green}")
                    .unwrap(),
            );

            pb.set_message(anidb_file.dub_language.clone());

            // CREATE TABLE IF NOT EXISTS indexed_files (
            //     path                TEXT PRIMARY KEY,
            //     filename            TEXT NOT NULL,
            //     filesize            INTEGER NOT NULL,
            //     ed2k                TEXT NOT NULL,
            //     fid                 INTEGER,
            //     first_seen          INTEGER NOT NULL,
            //     last_updated        INTEGER NOT NULL,

            //     UNIQUE (filename, filesize) ON CONFLICT REPLACE
            // );
        } else {
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {spinner:.green} {wide_msg:.yellow}")
                    .unwrap(),
            );

            pb.set_message(format!("Not found: {}", file_path.display()));
        }

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

        overall.inc(1);
        pb.finish();
    }

    overall.finish_with_message("Done!");

    crate::PROGRESS_BAR.write().unwrap().take();

    Ok(())
}
