use irkv::KvServer;
use irdb::SQLServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tokio::spawn(async {
    //     KvServer::start("0.0.0.0:8089").await.unwrap();
    // });

    tokio::spawn(async {
        SQLServer::start("0.0.0.0:8088").await.unwrap();
    });

    Ok(tokio::signal::ctrl_c().await?)
}
