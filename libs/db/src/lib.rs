use std::sync::atomic::AtomicU32;

use err::DBError;

mod err;
pub mod runtime;
pub mod config;

pub struct ReqContext {
    pub db_name: String,
    pub stmt_id: AtomicU32,
}

#[async_trait::async_trait]
pub trait IrDB {
    async fn init_db(&self, cx: &mut ReqContext, palyload: &[u8]) -> Result<(), DBError>;
    async fn query(&self, cx: &mut ReqContext, palyload: &[u8]) -> Result<(), DBError>;
    async fn prepare(&self, cx: &mut ReqContext, palyload: &[u8]) -> Result<(), DBError>;
    async fn execute(&self, cx: &mut ReqContext, palyload: &[u8]) -> Result<(), DBError>;
    async fn stmt_close(&self, cx: &mut ReqContext, palyload: &[u8]) -> Result<(), DBError>;
}
