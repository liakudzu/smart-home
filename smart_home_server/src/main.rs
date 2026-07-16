//! gRPC-сервер управления умным домом.

use std::net::SocketAddr;

use smart_home_proto::smart_home_server::SmartHomeServer;
use smart_home_server::SmartHomeService;
use tonic::transport::Server;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let addr: SocketAddr = std::env::var("SMART_HOME_GRPC_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:50051".to_string())
        .parse()?;

    let service = SmartHomeService::new();

    tracing::info!("gRPC smart home server listening on {addr}");

    Server::builder()
        .add_service(SmartHomeServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
