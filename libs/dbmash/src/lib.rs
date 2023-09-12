mod err;

use serde::{Deserialize, Serialize};

pub use err::Error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DBMashKind {
    MySQL,
    Natrue,
}

#[async_trait::async_trait]
pub trait DBMash {
    async fn start(&mut self) -> Result<(), Error>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MashConfig {
    pub listen_addr: String,
    pub db: String,
    pub user: String,
    pub password: String,
    pub mash_type: DBMashKind,
}

impl Default for MashConfig {
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0:8088".to_string(),
            db: "".to_string(),
            user: Default::default(),
            password: Default::default(),
            mash_type: DBMashKind::MySQL,
        }
    }
}
