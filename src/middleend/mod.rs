mod ir_error;
mod ir;
mod ir_block;

pub use ir_error::IrError;
pub use ir::*;
pub use ir_block::*;

pub mod ir_content;
pub mod ir_macros;
pub mod ir_metadata;
pub mod ir_resources;
pub mod ir_setup;
pub mod ir_variables;
