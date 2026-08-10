//! The resolved form of the configuration, which is what the rest of the
//! server runs on.
//!
//! Nothing here carries a `Located` wrapper, an `Option` on a setting the
//! server always needs, or a string the server would have to parse again. The
//! `Lower` impls the `Config` derive writes turn the spec into these types once
//! the error gate has passed.

use crate::conf::defaults;
use crate::conf::types::specification::server::{LimitsSpec, ServerSpec};
use crate::conf::validation::validator::net;
use confval::prelude::{keyword_enum, narrow, Config};
use std::net::IpAddr;

keyword_enum!(
    /// The tracing verbosity the server starts with.
    pub LogLevel,
    {
        Trace => "trace",
        Debug => "debug",
        Info  => "info",
        Warn  => "warn",
        Error => "error",
    }
);

/// The settings the server runs on.
#[derive(Debug, Config)]
#[confval(lower_from = ServerSpec)]
pub struct ServerConfig {
    #[confval(lower(from = bind_address, with = net::lower_ip_addr))]
    pub bind_address: IpAddr,

    #[confval(lower(from = port, with = narrow::i64_to_u16))]
    pub port: u16,

    #[confval(lower(from = log_level, with = narrow::keyword::<LogLevel>))]
    pub log_level: LogLevel,

    #[confval(nested, default)]
    pub limits: LimitsConfig,
}

/// The bounds the accept loop runs under.
#[derive(Debug, Clone, Copy, Config)]
#[confval(lower_from = LimitsSpec)]
pub struct LimitsConfig {
    #[confval(lower(from = max_connections, with = narrow::i64_to_usize))]
    pub max_connections: usize,

    #[confval(lower(from = max_backoff_seconds, with = narrow::i64_to_u64))]
    pub max_backoff_seconds: u64,
}

impl Default for LimitsConfig {
    fn default() -> Self {
        Self {
            max_connections: defaults::MAX_CONNECTIONS as usize,
            max_backoff_seconds: defaults::MAX_BACKOFF_SECONDS as u64,
        }
    }
}
