//! Unimarkup Errors used in [`unimarkup-rs`].
//!
//! [`frontend`]: crate::frontend
//! [`middleend`]: crate::middleend
//! [`backend`]: crate::backend
//! [`SyntaxError`]: crate::frontend::SyntaxError
//! [`IrError`]: crate::middleend::IrError
//! [`BackendError`]: crate::backend::BackendError
//! [`UnimarkupBlocks`]: crate::frontend::UnimarkupBlocks

mod error;

pub use error::UmError;
