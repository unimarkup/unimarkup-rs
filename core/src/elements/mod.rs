//! Available elements for a Unimarkup document.

mod heading_block;
mod metadata;
mod paragraph_block;
mod verbatim_block;

pub mod error;
pub mod inlines;
pub mod log_id;
pub mod types;

pub use heading_block::*;
pub use metadata::*;
pub use paragraph_block::*;
pub use verbatim_block::*;


use std::fmt;
use rusqlite::Transaction;
use logid::capturing::MappedLogId;
use unimarkup_render::render::Render;

use crate::{frontend::parser::UmParse, middleend::{AsIrLines, ContentIrLine, WriteToIr}, backend::ParseFromIr};

/// Used as a combined trait bound for all Unimarkup Elements.
pub trait UnimarkupBlock:
    Render + AsIrLines<ContentIrLine> + UmParse + ParseFromIr + fmt::Debug + WriteToIr
{
}

impl<T> UnimarkupBlock for T where
    T: Render + AsIrLines<ContentIrLine> + Clone + UmParse + ParseFromIr + fmt::Debug + WriteToIr
{
}

/// Type alias for a vector of elements that implement the [`UnimarkupBlock`] trait.
pub type UnimarkupBlocks = Vec<Box<dyn UnimarkupBlock>>;

impl WriteToIr for UnimarkupBlocks {
    fn write_to_ir(&self, ir_transaction: &Transaction) -> Result<(), MappedLogId> {
        for element in self {
            element.write_to_ir(ir_transaction)?;
        }

        Ok(())
    }
}
