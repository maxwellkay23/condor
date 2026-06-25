use clap::Parser;
use condor_common::proto::message_service_server::MessageServiceServer;

mod error;
mod nats;
mod redis_store;
mod service;

use error::CondorServerError;
use nats::{NatsConfig, NatsPublisher};
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
    #[arg(long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), CondorServerError> {
    let args = Args::parse();

    let store = RedisStore::connect(&args.redis_url, &args.root).await?;

    let publisher = if let Some(config_path) = &args.config {
        let contents = std::fs::read_to_string(config_path)?;
        let config: NatsConfig = serde_yaml::from_str(&contents)
            .map_err(|e| CondorServerError::Config(e.to_string()))?;
        match NatsPublisher::connect(&config).await {
            Ok(p) => {
                println!("nats connected to {}", config.url);
                Some(p)
            }
            Err(e) => {
                println!("nats connection failed (NATS disabled): {e}");
                None
            }
        }
    } else {
        None
    };

    let addr: std::net::SocketAddr = format!("{}:{}", args.host, args.port).parse()?;
    let svc = MessageServiceServer::new(MessageServiceImpl { store, publisher });

    println!("condor-server listening on {}:{}", args.host, args.port);

    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}
