//! The resolved form of the server configuration.
//!
//! The rest of the program reads these types. They carry no `Located` wrapper,
//! and every value already has the width and the type the runtime needs.

use confval::prelude::*;
use std::net::IpAddr;
use std::time::Duration;

use crate::conf::types::specification::server::{BackoffSpec, LogLevel, ServerSpec};
use crate::conf::validation::validator::net;

/// The server configuration the listener and the accept loop run on.
#[derive(Debug, Clone, confval::Config)]
#[confval(lower_from = ServerSpec)]
pub struct ServerConfig {
    /// IP address the listener binds to.
    #[confval(lower(from = bind, with = net::narrow_ip_addr))]
    pub bind: IpAddr,

    /// TCP port the listener accepts connections on.
    #[confval(lower(from = port, with = narrow::i64_to_u16))]
    pub port: u16,

    /// Number of connections the server serves at one time.
    #[confval(lower(from = max_connections, with = narrow::i64_to_usize))]
    pub max_connections: usize,

    /// Verbosity the tracing subscriber starts at.
    #[confval(lower(from = log_level, with = narrow::keyword::<LogLevel>))]
    pub log_level: LogLevel,

    /// Retry schedule the accept loop follows after a failed accept.
    #[confval(nested)]
    pub backoff: BackoffConfig,
}

/// The exponential retry schedule of the accept loop.
#[derive(Debug, Clone, Copy, confval::Config)]
#[confval(lower_from = BackoffSpec)]
pub struct BackoffConfig {
    /// Time the accept loop waits after the first failure.
    #[confval(lower(from = initial_seconds, with = narrow::i64_secs_to_duration))]
    pub initial: Duration,

    /// Time after which the accept loop gives up and returns the error.
    #[confval(lower(from = max_seconds, with = narrow::i64_secs_to_duration))]
    pub max: Duration,
}
