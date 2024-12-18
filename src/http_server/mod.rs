use std::sync::Arc;

use anyhow::Context as _;
use axum::{
    routing::{get, post},
    Router,
};
use tokio::{net::TcpListener, sync::RwLock};

use crate::anidb::Anidb;

mod error;
mod routes;

type Result<T> = std::result::Result<T, error::AppError>;

pub async fn run() -> anyhow::Result<()> {
    let anidb = Arc::new(RwLock::new(Anidb::new()));

    let app = Router::new()
        .route("/anime", get(routes::all_anime))
        .route("/anime/:aid", get(routes::anime))
        .route("/anime/:aid/episodes", get(routes::anime_episodes))
        .route("/anime/:aid/files", get(routes::anime_files))
        .route("/mpv", get(routes::mpv::mpv_upgrade))
        .route("/report-progress", post(routes::report_progress))
        .route("/settings", get(routes::settings::get))
        .route("/settings", post(routes::settings::post))
        .route("/animebytes/search", get(routes::proxy::animebytes::search))
        .route("/animebytes/groups/:id", get(routes::proxy::animebytes::group))
        .route("/animebytes/torrents/:id", get(routes::proxy::animebytes::torrent))
        .route("/platform_links", get(routes::platform_links::get))
        .with_state(anidb);

    let listener = TcpListener::bind("127.0.0.1:5352").await?;
    axum::serve(listener, app).await.context("Server error")
}
