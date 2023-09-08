use std::net::SocketAddr;

use anyhow::Result;
use bytes::BytesMut;
use futures::StreamExt;
use tokio::net::TcpListener;
use tokio_util::codec::Framed;
use tracing::{error, info};

use crate::server::{auth::{ServerHandshakeCodec, handshake}, stream::LocalStream, codec::PacketCodec};

mod auth;
mod codec;
mod stream;
mod tls;
mod err;

const SERVER_VERSION: &str = "5.7.37 pisa 0.1.0";

#[derive(Debug, Default)]
pub struct Server;

impl Server {
    pub async fn start(addr: &str) -> Result<()> {
        let addr: SocketAddr = addr.parse()?;

        let listener = TcpListener::bind(addr).await?;
        info!("Listening on {}", listener.local_addr()?);

        loop {
            let (socket, client) = listener.accept().await?;
            info!("new clinet connection {:?}", client);

            let user = "root".to_string();
            let password = "12345678".to_string();
            let db = "mydb".to_string();
            let server_version = SERVER_VERSION.to_string();

            let handshake_codec = ServerHandshakeCodec::new(user, password, db, server_version);
            let handshake_framed = Framed::with_capacity(LocalStream::from(socket), handshake_codec, 8196);

            tokio::spawn(async move {
                let res = handshake(handshake_framed).await;
                if let Err(err) = res {
                    error!("handshake error {:?}", err);
                    return;
                }

                let handshake_framed = res.unwrap().0;
                let parts = handshake_framed.into_parts();

                let packet_codec = PacketCodec::new(parts.codec, 8196);
                let io = parts.io;

                let mut framed = Framed::with_capacity(io, packet_codec, 16384);


                while let Some(res) = framed.next().await {
                    if let Err(err) = res {
                        error!("read framed error {:?}", err);
                        return;
                    }

                    let data = res.unwrap();

                    println!("-- {:?}", data);
                }



            });
        }

        async fn handle_connection(socket: tokio::net::TcpStream) -> Result<()> {
            Ok(())
        }

        
    }
}
