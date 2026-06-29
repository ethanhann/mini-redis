use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::time::Duration;

use confval::pipeline::narrow;
use confval::prelude::*;

use crate::conf::types::specification::ServerSpec;
use crate::conf::validation::validator::net::to_socket_addr;

#[derive(Debug, PartialEq, confval::Config)]
#[confval(lower_from = ServerSpec, validate)]
pub struct ServerConfig {
    #[confval(lower(from = (hostname, port), with = to_socket_addr))]
    pub addr: SocketAddr,

    #[confval(lower(from = max_connections, with = narrow::i64_to_usize))]
    pub max_connections: usize,

    #[confval(lower(from = shutdown_timeout_secs, with = narrow::i64_secs_to_duration))]
    pub shutdown_timeout: Duration,

    #[confval(lower(from = pub_sub_channel_capacity, with = narrow::i64_to_usize))]
    pub pub_sub_channel_capacity: usize,

    #[confval(lower(from = pid_file, with = to_pid_path))]
    pub pid_file: Option<PathBuf>,
}

impl ServerConfig {
    /// The subset of fields baked into the listener at bind time.
    /// Changes to these require a full restart.
    pub fn listener_identity(&self) -> (SocketAddr, usize) {
        (self.addr, self.max_connections)
    }
}

fn to_pid_path(value: &Option<Located<String>>, _report: &mut Report) -> Option<Option<PathBuf>> {
    Some(value.as_ref().map(|located| PathBuf::from(&located.value)))
}
