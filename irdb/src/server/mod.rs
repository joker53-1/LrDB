use std::net::SocketAddr;

use anyhow::{Ok, Result};
use db::{config::Config, runtime};

#[derive(Debug, Default)]
pub struct Server;

impl Server {
    pub async fn start(config: Config) -> Result<()> {
        let mut server = runtime::Server::new(config);
        server.start().await?;
        Ok(())
    }
}
