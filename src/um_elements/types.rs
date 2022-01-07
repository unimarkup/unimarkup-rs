//! Structs and enums representing the unimarkup type system.

use std::fmt;

use clap::ArgEnum;
use strum_macros::EnumString;

use crate::{
    backend::{ParseFromIr, Render},
    frontend::parser::UmParse,
    middleend::AsIrLines,
    um_elements,
};

use super::{HeadingBlock, ParagraphBlock};

/// Delimiter used in string representation of [`UnimarkupType`].
pub const DELIMITER: char = '-';

/// Used as a combined trait bound for all Unimarkup Elements.
pub trait UnimarkupBlock: Render + AsIrLines + UmParse + ParseFromIr + fmt::Debug {}

impl<T> UnimarkupBlock for T where T: Render + AsIrLines + UmParse + ParseFromIr + fmt::Debug {}

/// Type variants available in a Unimarkup document for Unimarkup content elements.
///
/// Each variant is briefly explained with short example. For more detailed
/// explanation of available unimarkup elements consult the
/// official [frontend reference of unimarkup](https://github.com/Unimarkup/Specification/blob/main/Frontend_Reference.markdown).
#[derive(Debug, PartialEq, Clone, EnumString, ArgEnum, strum_macros::Display)]
#[strum(ascii_case_insensitive, serialize_all = "kebab-case")]
pub enum UnimarkupType {
    /// A block of text surrounded with at least one blank line.
    ///
    /// Example:
    /// ```markdown
    /// This is a simple
    /// paragraph block
    /// ```
    Paragraph,

    /// A block of text surrounded with at least one blank line, and
    /// it starts with 1 to 6 `#` symbols.
    ///
    /// Example:
    /// ```markdown
    /// '# This is a heading with level 1'
    /// ```
    Heading,

    /// An unnumbered (unordered) list. Surrounded with at least one blank line.
    ///
    /// Example:
    /// ```markdown
    /// - List item 1
    /// - List item 2
    /// ```
    BulletList,

    /// A numbered (ordered) list. Surrounded with at least one blank line.
    ///
    /// Example:
    /// ```markdown
    /// 1. List item 1
    /// 2. List item 2
    /// ```
    NumberedList,

    /// A task list. Surrounded with at least one blank line.
    ///
    /// Example:
    /// ```markdown
    /// - [ ] List item 1
    /// - [a] List item 1
    /// - [/] List item 2
    /// ```
    TaskList,

    /// An unnumbered (unordered) list. Surrounded with at least one blank line.
    ///
    /// Example:
    /// ```markdown
    /// - List item ... with it's own definition
    /// - Another list item ... with another defintion
    /// ```
    DefinitionList,

    /// A table element. Surrounded with at least one blank line.
    ///
    /// Example: check the [specification](https://github.com/Unimarkup/Specification/blob/main/Frontend_Reference.markdown#table)
    Table,

    /// A verbatim block enclosed with 3 or more '~' symbols.
    ///
    /// Example:
    /// ```markdown
    /// ~~~ C
    /// int add(int a,  int b) {
    ///     return a + b;
    /// }
    /// ~~~
    /// ```
    VerbatimBlock,

    /// A render block enclosed with 3 or more `'` symbols.
    ///
    /// Example:
    /// ```markdown
    /// '''mermaid
    /// graph TB
    /// A & B--> C & D
    /// '''
    /// ```
    RenderBlock,

    /// A math block enclosed with 3 or more `$` symbols.
    ///
    /// Example:
    /// ```markdown
    /// $$$
    /// x = \frac{3}{4}
    /// $$$
    /// ```
    MathBlock,

    /// A figure insert block with syntax similar to markdown hyperlink,
    /// with `!!!` as prefix. Can be followed with unimarkup caption syntax. See example.
    ///
    /// Example:
    /// ```markdown
    /// !!![some image](<image url>).
    /// +++
    /// Image caption which explains the image.
    /// +++
    /// ```
    FigureInsert,

    /// A verbatim block which inserts whole file as is inside of it.
    ///
    /// Example:
    /// ```markdown
    /// ~~~[The whole style guide for Unimarkup](StyleGuide.markdown)
    /// ```
    VerbatimBlockInsert,

    /// A render block which inserts another file as render block.
    RenderBlockInsert,

    /// A text block enclosed between 3 or more pairs of `[]`
    ///
    /// Example:
    /// ```markdown
    /// [[[{<Attributes for the text block>}
    /// Everything inside is treated as Unimarkup content.
    /// Provided attributes apply to all text inside this block
    /// ]]]
    /// ```
    TextBlock,

    /// A block for quoting longer text. Denoted using `>` at
    /// the beginning of a line.
    ///
    /// Example:
    /// ```markdown
    /// > Some quoted text
    /// >
    /// >-- by someone
    /// >-- and many others
    /// >--{<author paragraph attributes>}
    /// ```
    QuotationBlock,

    /// Line blocks preserve all spaces, tabs and new lines. Denoted
    /// using `|` at the start of a line followed by a space.
    ///
    /// Example:
    /// ```markdown
    /// | Text where *spaces* are preserved as is.
    /// |    All other **markup** however, is considered as **Unimarkup text**.
    /// ```
    LineBlock,

    /// Definition blocks may be used to set a term with an optional classifier
    /// and a definition for this term. Denoted using `:` at the beginning of
    /// a line followed by a blank space.
    ///
    /// Example:
    /// ```markdown
    /// : Definition term :
    /// :
    /// : Definition of this term
    /// : may span multiple lines
    /// ```
    DefinitionBlock,

    /// It is possible to create explicit columns layout in unimarkup.
    /// For details and examples see explicit column block in [unimarkup specification](https://github.com/Unimarkup/Specification/blob/main/Frontend_Reference.md#explicit-column-block).
    ExplicitColumn,

    /// An implicit column block automatically splits its content by a given number of columns.
    /// For details and examples see explicit column block in [unimarkup specification](https://github.com/Unimarkup/Specification/blob/main/Frontend_Reference.md#implicit-column-block).
    ImplicitColumn,

    /// This element adds a field name at the start of a text block that is enclosed inside `:`.
    ///
    /// Example:
    /// ```markdown
    /// [[[:<field name>:
    ///
    /// Any Unimarkup content.
    ///
    /// ]]]
    /// ```
    FieldBlock,

    /// Every content inside an output block is forwarded as is to the rendered document.
    /// Enclosed with three or more `<` symbols.
    ///
    /// Example:
    /// ```markdown
    /// <<<
    /// <strong>Some important text</strong>
    /// <<<
    /// ```
    OutputBlock,

    /// Media insert blocks make it possible to insert video and audio files
    /// in addition to images using an extended figure insert syntax.
    ///
    /// For examples and more info see Media blocks in [unimarkup specification](https://github.com/Unimarkup/Specification/blob/main/Frontend_Reference.md#media-insert).
    MediaBlockInsert,

    /// If form blocks are allowed, predefined form macros may be used
    /// next to other Unimarkup content inside a form block to get user input.
    ///
    /// For examples and more info see Form blocks in [unimarkup specification](https://github.com/Unimarkup/Specification/blob/main/Frontend_Reference.md#form-block).
    FormBlock,

    /// It is possible to define macros in unimarkup. For more information see Macros section
    /// in [unimarkup specification](https://github.com/Unimarkup/Specification/blob/main/Frontend_Reference.md#macros).
    MacroDefinition,

    /// It is possible to define variables in unimarkup. For more information see Variables section
    /// in [unimarkup specification](https://github.com/Unimarkup/Specification/blob/main/Frontend_Reference.md#variables).
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
impl_from!(VerbatimBlock from um_elements::VerbatimBlock);

#[test]
fn check_if_converted() {
    let heading = HeadingBlock::default();

    let um_type = UnimarkupType::from(&heading);

    assert_eq!(um_type, UnimarkupType::Heading);
}
