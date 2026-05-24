use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::time::Duration;

#[derive(PartialEq)]
pub struct ServerConfig {
    pub addr: SocketAddr,
    pub max_connections: usize,
    pub shutdown_timeout: Duration,
    pub pid_file: Option<PathBuf>,
}

impl ServerConfig {
    /// The subset of fields baked into the listener at bind time.
    /// Changes to these require a full restart.
    pub fn listener_identity(&self) -> (SocketAddr, usize) {
        (self.addr, self.max_connections)
    }
}
