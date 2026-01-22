#![allow(dead_code)] // too much wip for now

use std::sync::RwLock as StdRwLock;

use anidb::Anidb;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use sqlx::SqlitePool;
use tokio::sync::RwLock;

use crate::config::Config;

pub mod anichart;
pub mod anidb;
pub mod animebytes;
pub mod config;
pub mod db;
pub mod gui;
pub mod http_server;
pub mod indexer;
pub mod log_proxy;
pub mod mpv;
pub mod remote_gui;
pub mod server;
pub mod ui;

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::read());
    pub static ref DB: AsyncOnce<SqlitePool> = AsyncOnce::new(db::init());
    pub static ref ANIDB: RwLock<Anidb> = RwLock::new(Anidb::new());
    pub static ref PROGRESS_BAR: StdRwLock<Option<indicatif::MultiProgress>> = StdRwLock::new(None);
}
