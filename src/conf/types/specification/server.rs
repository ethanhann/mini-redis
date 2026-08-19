//! The parsed form of the server configuration file.
//!
//! Every leaf is a `Located<T>`, so a diagnostic points at the line and column
//! the value came from. Each field holds the rawest type that parses without
//! failing. Narrowing happens at lowering.

use confval::prelude::*;

range_constraint!(PORT, i64, min: 1, max: 65535);
range_constraint!(MAX_CONNECTIONS, i64, min: 1, max: 65536);
range_constraint!(BACKOFF_SECONDS, i64, min: 1, max: 3600, units: "seconds");

keyword_enum!(
    /// Verbosity the tracing subscriber starts at.
    pub LogLevel,
    {
        Trace => "trace",
        Debug => "debug",
        Info => "info",
        Warn => "warn",
        Error => "error",
    }
);

/// The whole server configuration file.
#[derive(Debug, confval::Spec)]
#[confval(derive_default)]
pub struct ServerSpec {
    /// IP address the listener binds to.
    #[confval(default = "127.0.0.1".to_string())]
    pub bind: Located<String>,

    /// TCP port the listener accepts connections on.
    #[confval(default = 6379, range = PORT)]
    pub port: Located<i64>,

    /// Number of connections the server serves at one time.
    #[confval(default = 250, range = MAX_CONNECTIONS)]
    pub max_connections: Located<i64>,

    /// Verbosity the tracing subscriber starts at.
    #[confval(default = "info".to_string(), keywords = LogLevel)]
    pub log_level: Located<String>,

    /// Retry schedule the accept loop follows after a failed accept.
    #[confval(nested, default)]
    pub backoff: Located<BackoffSpec>,
}

/// The exponential retry schedule of the accept loop.
#[derive(Debug, confval::Spec)]
#[confval(derive_default)]
pub struct BackoffSpec {
    /// Seconds the accept loop waits after the first failure.
    #[confval(default = 1, range = BACKOFF_SECONDS)]
    pub initial_seconds: Located<i64>,

    /// Seconds after which the accept loop gives up and returns the error.
    #[confval(default = 64, range = BACKOFF_SECONDS)]
    pub max_seconds: Located<i64>,
}
