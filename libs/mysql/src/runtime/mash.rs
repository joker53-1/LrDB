use std::net::SocketAddr;

use bytes::BytesMut;
use dbmash::{DBMash, Error, MashConfig};
use futures::SinkExt;
use futures::StreamExt;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::net::TcpListener;
use tokio_util::codec::Decoder;
use tokio_util::codec::Encoder;
use tokio_util::codec::Framed;
use tracing::{debug, error, info};

use crate::err::ProtocolError;
use crate::server::codec::CommonPacket;
use crate::server::SERVER_VERSION;
use crate::server::{
    auth::{handshake, ServerHandshakeCodec},
    codec::{PacketCodec, PacketSend},
    stream::LocalStream,
};

pub struct MySQLMash {
    pub config: MashConfig,
    pub server_version: String,
}

impl MySQLMash {
    pub fn new(config: MashConfig) -> Self {
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
        let pck = PacketSend::Encode("MySQL Community Server (GPL)".as_bytes().into());
        framed.send(pck).await.unwrap();

        Ok(())
    }
}

#[async_trait::async_trait]
impl DBMash for MySQLMash {
    async fn start(&mut self) -> Result<(), Error> {
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
