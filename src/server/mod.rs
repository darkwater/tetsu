use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::{Context as _, Result};
use futures::{future, StreamExt};
use tarpc::{
    client,
    context::Context,
    server::{self, incoming::Incoming, Channel},
    tokio_serde::formats::Bincode,
};
use tokio::net::ToSocketAddrs;

use self::interface::{Error, TetsuServer, TetsuServerClient};
use crate::anidb::records::Anime;

pub mod interface;

pub async fn run() -> Result<()> {
    let server_addr = (IpAddr::V4(Ipv4Addr::UNSPECIFIED), 5352);

    let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Bincode::default).await?;
    log::info!("Listening on port {}", listener.local_addr().port());

    listener.config_mut().max_frame_length(usize::MAX);

    listener
        // Ignore accept errors.
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        // Limit channels to 1 per IP.
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        // serve is generated by the service attribute. It takes as input any type implementing
        // the generated World trait.
        .map(|channel| channel.execute(Server.serve()))
        // Max 10 channels.
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}

pub async fn connect<S: ToSocketAddrs>(addr: S) -> Result<TetsuServerClient> {
    let mut transport = tarpc::serde_transport::tcp::connect(addr, Bincode::default);
    transport.config_mut().max_frame_length(usize::MAX);

    // WorldClient is generated by the service attribute. It has a constructor `new` that takes a
    // config and any Transport as input.
    Ok(TetsuServerClient::new(tarpc::client::Config::default(), transport.await?).spawn())
}

#[derive(Clone)]
struct Server;

#[tarpc::server]
impl TetsuServer for Server {
    async fn anime(self, _: Context) -> Result<Vec<Anime>, Error> {
        let db = crate::DB.get().await;

        let mut anime = sqlx::query!("SELECT json FROM anime")
            .fetch_all(db)
            .await
            .context("Database query failed")?
            .into_iter()
            .map(|row| serde_json::from_str(&row.json).context("Invalid record in database"))
            .collect::<Result<Vec<Anime>>>()?;

        anime.sort_by(|a, b| a.romaji_name.cmp(&b.romaji_name));

        Ok(anime)
    }
}