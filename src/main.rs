use std::{path::PathBuf, sync::RwLock as StdRwLock};

use anidb::Anidb;
use anyhow::Result;
use async_once::AsyncOnce;
use clap::Parser;
use env_logger::Target;
use lazy_static::lazy_static;
use log_proxy::LogProxy;
use sqlx::SqlitePool;
use tokio::sync::RwLock;

use crate::config::Config;

mod anidb;
mod config;
mod db;
mod gui;
mod indexer;
mod log_proxy;
mod mpv;
mod server;
mod ui;

#[derive(Parser)]
#[clap(version, author, about)]
struct Args {
    #[clap(long)]
    index: Option<PathBuf>,

    #[clap(long)]
    login: bool,

    #[clap(long)]
    gui: bool,

    #[clap(long)]
    server: bool,
}

lazy_static! {
    static ref ARGS: Args = Args::parse();
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::read());
    static ref DB: AsyncOnce<SqlitePool> = AsyncOnce::new(db::init());
    static ref ANIDB: RwLock<Anidb> = RwLock::new(Anidb::new());
    static ref PROGRESS_BAR: StdRwLock<Option<indicatif::MultiProgress>> = StdRwLock::new(None);
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .target(Target::Pipe(Box::new(LogProxy)))
        .init();

    if ARGS.login {
        anidb::login().await
    } else if let Some(ref path) = ARGS.index {
        indexer::index(path).await
    } else if ARGS.gui {
        gui::run().await
    } else if ARGS.server {
        server::run().await
    } else {
        ui::run().await
    }
}
