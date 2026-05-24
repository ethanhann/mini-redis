use std::net::SocketAddr;
use tokio::time::Duration;

pub struct ServerConfig {
    pub addr: SocketAddr,
    pub max_connections: usize,
    pub shutdown_timeout: Duration,
}
