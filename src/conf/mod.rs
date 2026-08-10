//! Configuration, from the file on disk to the values the server runs on.
//!
//! The subsystem is built on confval and is split by the phase each piece
//! belongs to.
//!
//! * `types::specification` holds the parsed form, where every value keeps the
//!   span it came from.
//! * `validation` holds the rules, kept apart from the specs so a leaf check
//!   such as an address parse can be reused.
//! * `types::runtime` holds the resolved form the server runs on.
//! * `loader` runs the phases in order and reports what went wrong.
//!
//! Start a server with [`load`]. Pass `None` to take every default.

mod defaults;
pub mod loader;
pub mod types;
pub mod validation;

pub use loader::load;
pub use types::runtime::server::{LimitsConfig, LogLevel, ServerConfig};
pub use types::specification::server::{LimitsSpec, ServerSpec};
