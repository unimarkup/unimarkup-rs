#![warn(missing_docs)]
//! The unimarkup-rs crate is the official implementation of the [Unimarkup specification](https://github.com/Unimarkup/Specification/).

// TODO: set to private modules that don't have to be public
pub mod document;
pub mod elements;
pub mod metadata;
pub mod parser;
pub mod security;
pub mod unimarkup;
