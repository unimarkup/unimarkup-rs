use std::fmt;

use clap::ArgEnum;
use strum_macros::EnumString;

use crate::{
    backend::{ParseFromIr, Render},
    frontend::parser::UmParse,
    middleend::AsIrLines,
};

use super::{heading_block::HeadingBlock, paragraph_block::ParagraphBlock};

pub const DELIMITER: char = '-';

pub trait UnimarkupBlock: Render + AsIrLines + UmParse + ParseFromIr + fmt::Debug {}

impl<T> UnimarkupBlock for T where T: Render + AsIrLines + UmParse + ParseFromIr + fmt::Debug {}

#[derive(Debug, PartialEq, Clone, EnumString, ArgEnum, strum_macros::Display)]
#[strum(ascii_case_insensitive, serialize_all = "kebab-case")]
pub enum UnimarkupType {
    Paragraph,
    Heading,
    BulletList,
    NumberedList,
    TaskList,
    DefinitionList,
    Table,
    VerbatimBlock,
    RenderBlock,
    MathBlock,
    FigureInsert,
    VerbatimBlockInsert,
    RenderBlockInsert,
    TextBlock,
    QuotationBlock,
    LineBlock,
    DefinitionBlock,
    ExplicitColumn,
    ImplicitColumn,
    FieldBlock,
    OutputBlock,
    MediaBlockInsert,
    FormBlock,
    MacroDefinition,
    VariableDefinition,
}

/// Generate implementation of From<_> trait for UnimarkupType for a unimarkup block struct
///
/// ## Usage
///
/// ```rust
/// impl_from!(Heading from HeadingBlock);
///
/// // expands to
///
/// impl From<&HeadingBlock> for UnimarkupType {
///     fn from(_: &HeadingBlock) -> Self {
///         Self::Heading
///     }
/// }
/// ```
macro_rules! impl_from {
    ($($variant:ident from $struct:ty),*) => {
        $(
            impl From<&$struct> for UnimarkupType {
                fn from(_: &$struct) -> Self {
                    Self::$variant
                }
            }
        )*
    };
}

impl_from!(Heading from HeadingBlock);
impl_from!(Paragraph from ParagraphBlock);

#[test]
fn check_if_converted() {
    let heading = HeadingBlock::default();

    let um_type = UnimarkupType::from(&heading);

    assert_eq!(um_type, UnimarkupType::Heading);
}
