use std::net::SocketAddr;

use bytes::BytesMut;
use error::Error;
use futures::SinkExt;
use futures::StreamExt;
use protocol::err::ProtocolError;
use protocol::server::auth::handshake;
use protocol::server::auth::ServerHandshakeCodec;
use protocol::server::codec::CommonPacket;
use protocol::server::codec::PacketCodec;
use protocol::server::codec::PacketSend;
use protocol::server::stream::LocalStream;
use protocol::server::SERVER_VERSION;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::net::TcpListener;
use tokio_util::codec::Decoder;
use tokio_util::codec::Encoder;
use tokio_util::codec::Framed;
use tracing::{debug, error, info};

use crate::config::Config;

pub struct Server {
    pub config: Config,
    pub server_version: String,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            server_version: SERVER_VERSION.to_owned(),
        }
    }
    async fn handle_command<T, C>(framed: &mut Framed<T, C>, data: BytesMut) -> Result<(), Error>
    where
        T: AsyncRead + AsyncWrite + Unpin,
        C: Decoder<Item = BytesMut, Error = ProtocolError>
            + Encoder<PacketSend<Box<[u8]>>, Error = ProtocolError>
            + CommonPacket,
    {
        let version = vec![
            1, 0, 0, 1, 1, 39, 0, 0, 2, 3, 100, 101, 102, 0, 0, 0, 17, 64, 64, 118, 101, 114, 115,
            105, 111, 110, 95, 99, 111, 109, 109, 101, 110, 116, 0, 12, 45, 0, 112, 0, 0, 0, 253,
            0, 0, 31, 0, 0, 5, 0, 0, 3, 254, 0, 0, 2, 0, 29, 0, 0, 4, 28, 77, 121, 83, 81, 76, 32,
            67, 111, 109, 109, 117, 110, 105, 116, 121, 32, 83, 101, 114, 118, 101, 114, 32, 40,
            71, 80, 76, 41, 5, 0, 0, 5, 254, 0, 0, 2, 0,
        ];
        // let pck = PacketSend::Encode("MySQL Community Server (GPL)".as_bytes().into());
        let pck = PacketSend::Origin(version.into());
        framed.send(pck).await.unwrap();

        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        let listener = TcpListener::bind(&self.config.listen_addr).await?;
        info!("Listening on {}", listener.local_addr()?);

        loop {
            let (socket, client) = listener.accept().await?;
            info!("new clinet connection {:?}", client);

            let handshake_codec = ServerHandshakeCodec::new(
                self.config.user.clone(),
                self.config.password.clone(),
                self.config.db.clone(),
                self.server_version.clone(),
            );
            let handshake_framed =
                Framed::with_capacity(LocalStream::from(socket), handshake_codec, 8196);

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

                    debug!("new commond: {:?}", data);
                    if let Err(err) = Self::handle_command(&mut framed, data).await {
                        error!("read framed error {:?}", err);
                        return;
                    }
                }
            });
        }

        Ok(())
    }
}
