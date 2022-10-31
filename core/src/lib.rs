#![warn(missing_docs)]
//! The unimarkup-rs crate is the official implementation of the [Unimarkup specification](https://github.com/Unimarkup/Specification/).

pub mod backend;
pub mod config;
pub mod document;
pub mod elements;
pub mod error;
pub mod frontend;
pub mod highlight;
pub mod log_id;
pub mod middleend;
pub mod security;
pub mod unimarkup;
pub mod unimarkup_block;
