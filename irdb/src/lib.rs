use async_trait::async_trait;

#[macro_use]
extern crate lazy_static;

mod server;
mod mysql;
mod err;
mod util;
mod charset;
mod session;

pub use server::Server as SQLServer;

#[async_trait]
pub trait Database: Sync + Send {
    async fn verify_account(&self, username: &str, password: &str) -> anyhow::Result<()>;
}