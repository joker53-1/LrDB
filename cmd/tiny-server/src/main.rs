use kv::KvServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    KvServer::start("[::1]:50051").await
}