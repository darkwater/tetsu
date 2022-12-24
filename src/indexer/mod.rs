use std::{path::Path, time::Duration};

use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tokio::fs;

use crate::db;

mod ed2k;

pub async fn index(path: &Path) -> Result<()> {
    let db = db::init().await?;

    let mpb = MultiProgress::new();

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

        let ed2k = tokio::task::block_in_place(|| {
            ed2k::hash_file(&file_path, &pb).unwrap();
        });

        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {spinner:.green} {wide_msg:.yellow}")
                .unwrap(),
        );

        pb.set_message(format!("Not found: {}", file_path.display()));

        overall.inc(1);
        pb.finish();
    }

    overall.finish_with_message("Done!");

    Ok(())
}
