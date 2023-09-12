use irkv::KvServer;
use irdb::SQLServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {
        KvServer::start("[::1]:50051").await.unwrap();
    });

    tokio::spawn(async {
        SQLServer::start("[::1]:50052").await.unwrap();
    });

    Ok(tokio::signal::ctrl_c().await?)
}
