#![warn(missing_docs)]
//! The unimarkup-rs crate is the official implementation of the [Unimarkup specification](https://github.com/Unimarkup/Specification/).

pub mod backend;
pub mod config;
pub mod document;
pub mod elements;
pub mod frontend;
pub mod log_id;
pub mod metadata;
pub mod middleend;
pub mod security;
pub mod unimarkup;
