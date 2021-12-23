//! Unimarkup Error used in [`frontend`], [`backend`] and [`middleend`] modules.
//!
//! It's used to describe various errors which can occur during the compilation
//! of unimarkup document, e.g. [`SyntaxError`] when parsing the document, [`IrError`]
//! when communication with IR fails, or [`BackendError`] when constructing
//! [`UnimarkupBlocks`] in the [`backend`].
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
