use db::config::Config;
use irdb::SQLServer;
use irkv::KvServer;
use tracing::Level;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();

    tokio::spawn(async {
        KvServer::start("0.0.0.0:8089").await.unwrap();
    });

    let config = Config {
        db: "mydb".to_string(),
        user: "root".to_string(),
        password: "12345678".to_string(),
        listen_addr: "0.0.0.0:8088".to_string(),
    };

    tokio::spawn(async {
        SQLServer::start(config).await.unwrap();
    });

    Ok(tokio::signal::ctrl_c().await?)
}
