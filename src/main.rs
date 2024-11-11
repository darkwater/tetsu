#![allow(dead_code)] // too much wip for now

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
mod remote_gui;
mod server;
mod ui;

#[derive(Parser)]
#[clap(version, author, about)]
struct Args {
    /// Enable remote control
    #[clap(long)]
    server: Option<ServerType>,

    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(Parser, Default)]
enum Subcommand {
    /// Login to AniDB
    Login,

    /// Index a directory of anime files
    Index { path: PathBuf },

    /// Run the TUI
    #[default]
    Tui,

    /// Run the GUI
    Gui,

    /// Connect to a remote Tetsu instance
    RemoteGui { addr: String },
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
    if ARGS.server.is_some() && ARGS.subcommand.is_none() && std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::builder()
        .target(Target::Pipe(Box::new(LogProxy)))
        .init();

    let server_handle = ARGS.server.as_ref().map(|stype| {
        tokio::spawn(async move {
            anichart::linker::run().await;
        });

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

    match ARGS.subcommand {
        None if ARGS.server.is_some() => {}
        Some(Subcommand::Login) => {
            anidb::login().await?;
        }
        Some(Subcommand::Index { ref path }) => {
            indexer::index(path).await?;
        }
        None | Some(Subcommand::Tui) => {
            ui::run().await?;

            if let Some(ref handle) = server_handle {
                handle.abort();
            }
        }
        Some(Subcommand::Gui) => {
            gui::run().unwrap();
        }
        Some(Subcommand::RemoteGui { ref addr }) => {
            remote_gui::run(addr).await?;
        }
    }

    if let Some(handle) = server_handle {
        let res = handle.await;

        if let Err(e) = res {
            if !e.is_cancelled() {
                Err(e)?;
            }
        }
    }

    Ok(())
}
