use bytes::{Buf, BytesMut};
use error::Error;
use futures::SinkExt;
use futures::StreamExt;
use protocol::err::ProtocolError;
use protocol::mysql_const::ComType;
use protocol::server::auth::handshake;
use protocol::server::auth::ServerHandshakeCodec;
use protocol::server::codec::PacketCodec;
use protocol::server::codec::PacketSend;
use protocol::server::codec::{make_err_packet, CommonPacket};
use protocol::server::err::MySQLError;
use protocol::server::stream::LocalStream;
use protocol::server::SERVER_VERSION;
use std::time::{Duration, Instant};
use std::{marker::PhantomData, sync::atomic::AtomicU32};
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::net::TcpListener;
use tokio_util::codec::Decoder;
use tokio_util::codec::Encoder;
use tokio_util::codec::Framed;
use tracing::{debug, error, info};
use tracing_subscriber::registry::Data;

use crate::DBError;

pub struct ReqContext<T, C> {
    pub framed: Framed<T, C>,
    pub stmt_id: AtomicU32,
}

pub struct Resp {
    pub ep: Option<String>,
    pub duration: Duration,
}

#[async_trait::async_trait]
pub trait IrDBService<T, C> {
    async fn init_db(&self, cx: &mut ReqContext<T, C>, palyload: &[u8]) -> Result<(), Error>;
    async fn query(&self, cx: &mut ReqContext<T, C>, palyload: &[u8]) -> Result<(), Error>;
    async fn prepare(&self, cx: &mut ReqContext<T, C>, palyload: &[u8]) -> Result<(), Error>;
    async fn execute(&self, cx: &mut ReqContext<T, C>, palyload: &[u8]) -> Result<(), Error>;
    async fn stmt_close(&self, cx: &mut ReqContext<T, C>, palyload: &[u8]) -> Result<(), Error>;
}

pub struct IrDBInstance<S, T, C> {
    inner: S,
    is_quit: bool,
    plantom: PhantomData<(T, C)>,
}

impl<S, T, C> IrDBInstance<S, T, C>
where
    // S: IrDBService<T, C>,
    T: AsyncRead + AsyncWrite + Unpin,
    C: Decoder<Item = BytesMut, Error = ProtocolError>
        + Encoder<PacketSend<Box<[u8]>>, Error = ProtocolError>
        + CommonPacket,
{
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            is_quit: false,
            plantom: PhantomData,
        }
    }

    pub async fn run(&mut self, cx: &mut ReqContext<T, C>) -> Result<(), Error> {
        while let Some(data) = cx.framed.next().await {
            let data = data?;

            if let Err(err) = self.handle_command(cx, data).await {
                let err_info = make_err_packet(MySQLError {
                    code: 2002,
                    state: "HY000".as_bytes().to_vec(),
                    msg: err.to_string(),
                });
                cx.framed
                    .send(PacketSend::Encode(err_info[4..].into()))
                    .await?;
                error!("exec command error: {:?}", err);
            }
        }
        cx.framed.codec_mut().reset_seq();

        if self.is_quit {
            return Ok(());
        }

        Ok(())
    }

    async fn handle_command(
        &mut self,
        cx: &mut ReqContext<T, C>,
        mut data: BytesMut,
    ) -> Result<Resp, Error> {
        let now = Instant::now();
        let command = data.get_u8();
        let palyload = data.split();

        match ComType::from(command) {
            ComType::QUIT => {
                self.is_quit = true;
                Ok(Resp {
                    ep: None,
                    duration: now.elapsed(),
                })
            }
            ComType::INIT_DB => todo!(),
            ComType::QUERY => {
                self.query(cx, &palyload).await?;
                Ok(Resp {
                    ep: None,
                    duration: now.elapsed(),
                })
            }
            ComType::FIELD_LIST => todo!(),
            ComType::CREATE_DB => todo!(),
            ComType::DROP_DB => todo!(),
            ComType::PING => Ok(Resp {
                ep: None,
                duration: now.elapsed(),
            }),
            ComType::STMT_PREPARE => todo!(),
            ComType::STMT_EXECUTE => todo!(),
            ComType::STMT_CLOSE => todo!(),
            ComType::STMT_RESET => todo!(),
            x => {
                todo!()
            }
        }
    }

    async fn query(&self, cx: &mut ReqContext<T, C>, palyload: &[u8]) -> Result<(), Error> {
        debug!("$: {:?}", String::from_utf8_lossy(palyload));
        let version = vec![
            1, 0, 0, 1, 1, 39, 0, 0, 2, 3, 100, 101, 102, 0, 0, 0, 17, 64, 64, 118, 101, 114, 115,
            105, 111, 110, 95, 99, 111, 109, 109, 101, 110, 116, 0, 12, 45, 0, 112, 0, 0, 0, 253,
            0, 0, 31, 0, 0, 5, 0, 0, 3, 254, 0, 0, 2, 0, 29, 0, 0, 4, 28, 77, 121, 83, 81, 76, 32,
            67, 111, 109, 109, 117, 110, 105, 116, 121, 32, 83, 101, 114, 118, 101, 114, 32, 40,
            71, 80, 76, 41, 5, 0, 0, 5, 254, 0, 0, 2, 0,
        ];
        // let pck = PacketSend::Encode("MySQL Community Server (GPL)".as_bytes().into());
        let pck = PacketSend::Origin(version.into());
        cx.framed.send(pck).await.unwrap();
        Ok(())
    }
}
