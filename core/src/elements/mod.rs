//! Available elements for a Unimarkup document.

pub mod atomic;
pub mod enclosed;
pub mod inlines;
pub mod log_id;
pub mod preamble;
pub mod types;

use logid::capturing::MappedLogId;
use rusqlite::Transaction;
use std::fmt;
use unimarkup_render::render::Render;

use crate::{
    backend::ParseFromIr,
    frontend::parser::UmParse,
    middleend::{AsIrLines, ContentIrLine, WriteToIr},
};

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
