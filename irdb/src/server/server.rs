use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;

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
use crate::server::irdb::IrDBInstance;
use crate::server::irdb::ReqContext;

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

    pub async fn start(&self) -> Result<(), Error> {
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

            let mut ins = IrDBInstance::new("DB service");

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

                let framed = Framed::with_capacity(io, packet_codec, 16384);
                let mut cx = ReqContext {
                    framed,
                    stmt_id: AtomicU32::new(0),
                };

                if let Err(err) = ins.run(&mut cx).await {
                    error!("instance run error {:?}", err);
                }
            });
        }
    }
}
