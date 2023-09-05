use std::net::SocketAddr;

use anyhow::Result;
use tokio::net::TcpListener;

#[derive(Debug, Default)]
pub struct Server;

impl Server {
    pub async fn start(addr: &str) -> Result<()> {
        let addr: SocketAddr = addr.parse()?;

        let listener = TcpListener::bind(addr).await?;
        println!("Listening on {}", listener.local_addr()?);

        loop {
            let (socket, _) = listener.accept().await?;
            tokio::spawn(async move {
                if let Err(err) = handle_connection(socket).await {
                    eprintln!("Error handling connection: {}", err);
                }
            });
        }

        async fn handle_connection(socket: tokio::net::TcpStream) -> Result<()> {
            Ok(())
        }
    }
}
