//! The rules the parsed configuration is checked against.
//!
//! `validator` holds leaf checks that carry no mini-redis knowledge, such as
//! parsing an address. The per-area modules hold the `Validate` impls, which
//! call those leaf checks and add the rules that are specific to mini-redis.
//! Keeping the two apart lets a leaf check serve more than one field.

mod server;
pub mod validator;
