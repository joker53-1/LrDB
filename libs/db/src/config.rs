use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub listen_addr: String,
    pub db: String,
    pub user: String,
    pub password: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0:8088".to_string(),
            db: "".to_string(),
            user: Default::default(),
            password: Default::default(),
        }
    }
}
