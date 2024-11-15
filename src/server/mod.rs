use std::net::{IpAddr, Ipv4Addr};

use anyhow::Result;
use futures::StreamExt;
use tarpc::{
    server::{BaseChannel, Channel},
    tokio_serde::formats::Bincode,
};
use tokio::net::ToSocketAddrs;

use self::interface::{TetsuServer, TetsuServerClient};

mod ifimpl;
pub mod interface;

pub async fn run() -> Result<()> {
    let server_addr = (IpAddr::V4(Ipv4Addr::UNSPECIFIED), 5352);
    let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Bincode::default).await?;

    log::info!("Listening on port {}", listener.local_addr().port());

    while let Some(sock) = listener.next().await {
        log::debug!("New connection");

        match sock {
            Ok(transport) => {
                tokio::spawn(
                    BaseChannel::with_defaults(transport)
                        .execute(self::ifimpl::Server.serve())
                        .for_each(|response| async move {
                            tokio::spawn(response);
                        }),
                );
            }
            Err(e) => {
                log::error!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}

pub async fn connect<S: ToSocketAddrs>(addr: S) -> Result<TetsuServerClient> {
    let mut transport = tarpc::serde_transport::tcp::connect(addr, Bincode::default);
    transport.config_mut().max_frame_length(usize::MAX);

    // WorldClient is generated by the service attribute. It has a constructor `new` that takes a
    // config and any Transport as input.
    Ok(TetsuServerClient::new(tarpc::client::Config::default(), transport.await?).spawn())
}
