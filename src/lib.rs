#![warn(missing_docs)]
//! unimarkup-rs crate is the official compiler implementation of the [unimarkup](https://github.com/Unimarkup/Specification/) compiler.

pub mod backend;
pub mod config;
pub mod frontend;
pub mod middleend;
pub mod um_elements;
pub mod um_error;
pub mod unimarkup;
