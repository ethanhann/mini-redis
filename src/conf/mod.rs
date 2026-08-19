//! Configuration for the mini-redis server.
//!
//! A configuration file moves through four phases. Parsing produces the spec,
//! where every leaf keeps the span it came from. Validation fills a report with
//! every problem in one pass. The gate stops the load while the report holds
//! errors. Lowering produces the runtime types the server runs on.
//!
//! The layers are separate. `types::specification` holds the parsed shape,
//! `validation` holds the rules, and `types::runtime` holds the resolved form.

pub mod loader;
pub mod types;
pub mod validation;

pub use loader::{defaults, load, load_text, LoadError};
pub use types::runtime::server::{BackoffConfig, ServerConfig};
pub use types::specification::server::{BackoffSpec, LogLevel, ServerSpec};
