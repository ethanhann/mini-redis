use crate::Error;
use std::convert::TryFrom;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::time::Duration;

use super::server_config::ServerConfig;
use crate::conf::types::specification::server::ServerSpec;

impl TryFrom<ServerSpec> for ServerConfig {
    type Error = Error;

    fn try_from(spec: ServerSpec) -> Result<Self, Self::Error> {
        let addr: SocketAddr = format!("{}:{}", spec.hostname, spec.port).parse()?;
        Ok(ServerConfig {
            addr,
            max_connections: spec.max_connections,
            shutdown_timeout: Duration::from_secs(spec.shutdown_timeout_secs),
            pid_file: spec.pid_file.map(PathBuf::from),
        })
    }
}
