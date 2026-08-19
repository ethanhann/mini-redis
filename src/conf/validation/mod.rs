//! Semantic rules, kept apart from the types they check.
//!
//! `validator` holds reusable leaf checks. `server` holds the rules for the
//! server spec, which are the format check and the cross-field rule that no
//! field attribute can express.

pub mod server;
pub mod validator;
