use clap::Parser;
use condor_common::proto::message_service_server::MessageServiceServer;

mod error;
mod redis_store;
mod service;

use error::CondorServerError;
use redis_store::RedisStore;
use service::MessageServiceImpl;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    #[arg(long, default_value_t = 8080)]
    port: u16,
    #[arg(long, default_value = "redis://127.0.0.1/")]
    redis_url: String,
    #[arg(long, default_value = "")]
    root: String,
}

#[tokio::main]
async fn main() -> Result<(), CondorServerError> {
    let args = Args::parse();

    let store = RedisStore::connect(&args.redis_url, &args.root).await?;

    let addr: std::net::SocketAddr = format!("{}:{}", args.host, args.port).parse()?;
    let svc = MessageServiceServer::new(MessageServiceImpl { store });

    println!("condor-server listening on {}:{}", args.host, args.port);

    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}
