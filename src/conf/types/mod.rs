//! The two forms every setting takes.
//!
//! A setting appears once in [`specification`], span-tracked and holding the
//! rawest type the file produces, and once in [`runtime`], narrowed to the
//! type the server uses. Lowering is what carries a value from one to the
//! other.

pub mod runtime;
pub mod specification;
