//! Security functionality of [`unimarkup-rs`](crate).
//!
//! i.e. hashing

pub mod error;
pub mod log_id;

mod hashing;
pub use hashing::*;
