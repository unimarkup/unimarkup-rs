//! Structs, traits and helper functions for interaction between [`frontend`] and
//! [`backend`].
//!
//! It serves as an interface between [`frontend`] and [`backend`], and enables interaction with the
//! Intermediate Representation (IR). Through middleend it is possible both to store and retrieve
//! data to and from IR (Sqlite).
//!
//! [`backend`]: crate::backend
//! [`frontend`]: crate::frontend

mod ir;
mod ir_block;
mod ir_content;
mod ir_error;
mod ir_macros;
mod ir_metadata;
mod ir_resources;
mod ir_setup;
mod ir_variables;

pub use ir::*;
pub use ir_block::*;
pub use ir_content::*;
pub use ir_error::IrError;
pub use ir_macros::*;
pub use ir_metadata::*;
pub use ir_resources::*;
pub use ir_setup::*;
pub use ir_variables::*;
