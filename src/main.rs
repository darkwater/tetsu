use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use env_logger::Target;
use tetsu::{log_proxy::LogProxy, *};

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
    Index {
        /// Folder or file to index
        path: PathBuf,

        /// Write a .m3u8 playlist file
        #[clap(short, long)]
        write_playlist: Option<PathBuf>,

        /// Dump AniDB data to a JSON file
        #[clap(short, long)]
        json_dump: Option<PathBuf>,
    },

    /// Run the TUI
    #[default]
    Tui,

    /// Run the GUI
    Gui,

    /// Connect to a remote Tetsu instance
    RemoteGui { addr: String },
}

#[derive(Clone, Copy, ValueEnum)]
enum ServerType {
    Tarpc,
    Http,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.server.is_some() && args.subcommand.is_none() && std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::builder()
        .target(Target::Pipe(Box::new(LogProxy)))
        .init();

    let server_handle = args.server.map(|stype| {
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

    match &args.subcommand {
        None if args.server.is_some() => {}
        Some(Subcommand::Login) => {
            anidb::login().await?;
        }
        Some(Subcommand::Index { path, write_playlist, json_dump }) => {
            indexer::index(path).await?;

            if let Some(playlist) = write_playlist {
                indexer::playlist::write(path, playlist).await?;
            }

            if let Some(json_path) = json_dump {
                indexer::dump::dump_json(path, json_path).await?;
            }
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
        Some(Subcommand::RemoteGui { addr }) => {
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
