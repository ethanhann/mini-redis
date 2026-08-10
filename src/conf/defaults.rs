//! The value each setting takes when the configuration file omits it.
//!
//! Each constant is declared once here and read from both sides of the
//! pipeline. The spec names it in a `#[confval(default = ...)]` attribute, and
//! the runtime `Default` impls narrow the same constant, so the value the
//! parser fills and the value a caller gets without a file cannot drift apart.
//!
//! The types are the spec's types rather than the runtime's, because the
//! attribute expressions are what the parser stores.

/// The loopback interface, which keeps a server started without a
/// configuration file off the network.
pub(crate) const BIND_ADDRESS: &str = "127.0.0.1";

/// The port redis listens on by convention.
pub(crate) const PORT: i64 = 6379;

/// The verbosity `RUST_LOG` overrides when it is set.
pub(crate) const LOG_LEVEL: &str = "info";

/// The number of concurrent connections the listener accepts before it waits
/// for one to terminate.
///
/// This is set low to discourage running mini-redis in production.
pub(crate) const MAX_CONNECTIONS: i64 = 250;

/// The longest the accept loop waits between retries before it gives up.
pub(crate) const MAX_BACKOFF_SECONDS: i64 = 64;
