//! The parsed form of a mini-redis configuration file.
//!
//! Every leaf is a `Located<T>` holding the rawest type TOML produces, so each
//! value keeps the byte range it came from and a diagnostic can point at it.
//! Narrowing a port to a `u16` or a level keyword to an enum happens at
//! lowering, in [`crate::conf::types::runtime::server`].
//!
//! The doc comment on each field is rendered above that field by
//! `to_template`, so it is what an operator reads in a generated starter file.

use crate::conf::defaults;
use confval::prelude::{Located, Spec};

/// The settings the server reads at startup.
#[derive(Debug, Spec)]
#[confval(derive_default)]
pub struct ServerSpec {
    /// The IP address the listener binds to. Use 0.0.0.0 to accept
    /// connections from any interface.
    #[confval(default = defaults::BIND_ADDRESS.to_string())]
    pub bind_address: Located<String>,

    /// The TCP port the listener binds to.
    #[confval(default = defaults::PORT)]
    pub port: Located<i64>,

    /// The tracing verbosity, one of trace, debug, info, warn, or error.
    /// Setting RUST_LOG overrides this.
    #[confval(default = defaults::LOG_LEVEL.to_string())]
    pub log_level: Located<String>,

    /// Bounds on what the accept loop will do. Omit the block to take every
    /// default.
    #[confval(nested)]
    pub limits: Option<Located<LimitsSpec>>,
}

/// The bounds the accept loop runs under.
#[derive(Debug, Spec)]
#[confval(derive_default)]
pub struct LimitsSpec {
    /// How many connections may be open at once. The listener stops
    /// accepting once this many are active and resumes when one closes.
    #[confval(default = defaults::MAX_CONNECTIONS)]
    pub max_connections: Located<i64>,

    /// How long the accept loop may wait between retries. The wait starts at
    /// one second and doubles after each failure. Once a wait would exceed
    /// this, the server stops rather than retrying again.
    #[confval(default = defaults::MAX_BACKOFF_SECONDS)]
    pub max_backoff_seconds: Located<i64>,
}
