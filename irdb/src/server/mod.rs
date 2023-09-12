use std::net::SocketAddr;

use anyhow::{Ok, Result};
use dbmash::{DBMash, MashConfig};
use mysql::runtime::MySQLMash;

#[derive(Debug, Default)]
pub struct Server;

impl Server {
    pub async fn start(config: MashConfig) -> Result<()> {
        match config.mash_type {
            dbmash::DBMashKind::MySQL => {
                let mut mash = MySQLMash::new(config);
                mash.start().await.unwrap();
                Ok(())
            }
            dbmash::DBMashKind::Natrue => todo!(),
        }
    }
}
