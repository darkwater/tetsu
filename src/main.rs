use std::{path::PathBuf, sync::RwLock as StdRwLock};

use anidb::Anidb;
use anyhow::Result;
use async_once::AsyncOnce;
use clap::{Parser, ValueEnum};
use env_logger::Target;
use lazy_static::lazy_static;
use log_proxy::LogProxy;
use sqlx::SqlitePool;
use tokio::sync::RwLock;

use crate::config::Config;

mod anichart;
mod anidb;
mod animebytes;
mod config;
mod db;
mod gui;
mod http_server;
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
    server: Option<ServerType>,

    /// Combine with --server to disable the TUI
    #[clap(long)]
    no_ui: bool,
}

#[derive(Clone, ValueEnum)]
enum ServerType {
    Tarpc,
    Http,
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
    if ARGS.server.is_some() && ARGS.no_ui && std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::builder()
        .target(Target::Pipe(Box::new(LogProxy)))
        .init();

    let server_handle = ARGS.server.as_ref().map(|stype| {
        tokio::spawn(async move {
            let res = match stype {
                ServerType::Tarpc => server::run().await,
                ServerType::Http => http_server::run().await,
            };

            if let Err(e) = res {
                log::error!("Server error: {}", e);
                log::error!("Server shutting down");
            }
        })
    });

    if ARGS.login {
        anidb::login().await?;
    } else if let Some(ref path) = ARGS.index {
        indexer::index(path).await?;
    } else if ARGS.gui {
        gui::run().await?;
    } else {
        ui::run().await?;
    }

    if let Some(handle) = server_handle {
        handle.await?;
    }

    Ok(())
}
