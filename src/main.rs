use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use lazy_static::lazy_static;
use tokio::sync::RwLock;

use crate::config::Config;

mod config;
mod db;
mod indexer;

#[derive(Parser)]
#[clap(version, author, about)]
struct Args {
    #[clap(long)]
    index: Option<PathBuf>,
}

lazy_static! {
    static ref ARGS: Args = Args::parse();
    static ref CONFIG: RwLock<Config> = RwLock::new(Config::read());
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(path) = args.index {
        indexer::index(&path).await?;
    }

    Ok(())
}
