//! Structs and enums representing the unimarkup type system.

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

/// Delimiter used in string representation of [`ElementType`].
pub const ELEMENT_TYPE_DELIMITER: char = '-';

/// Type variants available in a Unimarkup document for Unimarkup content elements.
///
/// Each variant is briefly explained with short example. For more detailed
/// explanation of available Unimarkup elements consult the
/// official [frontend reference of Unimarkup](https://github.com/Unimarkup/Specification/tree/main/Frontend).
#[derive(
    Debug, PartialEq, Eq, Clone, EnumString, strum_macros::Display, Serialize, Deserialize,
)]
#[strum(ascii_case_insensitive, serialize_all = "kebab-case")]
pub enum ElementType {
    /// A block of text.
    ///
    /// Example:
    /// ```markdown
    /// This is a simple
    /// paragraph block
    /// ```
    Paragraph,

    /// A line of text starting with 1 to 6 `#` symbols.
    ///
    /// Example:
    /// ```markdown
    /// '# This is a heading with level 1'
    /// ```
    Heading,

    /// An unnumbered (unordered) list.
    ///
    /// Example:
    /// ```markdown
    /// - List item 1
    /// - List item 2
    /// ```
    BulletList,

    /// A numbered (ordered) list.
    ///
    /// Example:
    /// ```markdown
    /// 1. List item 1
    /// 2. List item 2
    /// ```
    NumberedList,

    /// A task list.
    ///
    /// Example:
    /// ```ignore
    /// -[ ] List item 1
    /// -[a] List item 1
    /// -[/] List item 2
    /// ```
    TaskList,

    /// An unnumbered (unordered) list with a definition term (left), description (right) and optional class (directly after `...`).
    ///
    /// Example:
    /// ```markdown
    /// - List item ... with it's own definition
    /// - Another list item ... with another defintion
    /// ```
    DefinitionList,

    /// A table element.
    ///
    /// Example: check the [specification](https://github.com/Unimarkup/Specification/blob/main/Frontend/AtomicBlocks.md#table)
    Table,

    /// A verbatim block enclosed with 3 or more '~' symbols.
    ///
    /// Example:
    /// ````markdown
    /// ```C
    /// int add(int a,  int b) {
    ///     return a + b;
    /// }
    /// ```
    /// ````
    Verbatim,

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
    /// with `!!!` as prefix. Can be followed with Unimarkup caption syntax (See example).
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
    /// ~~~[The whole style guide for Unimarkup](StyleGuide.md)
    /// ```
    VerbatimInsert,

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
    /// For details and examples see explicit column block in the [Unimarkup specification](https://github.com/Unimarkup/Specification/blob/main/Frontend/EnclosedBlocks.md#explicit-column-block).
    ExplicitColumn,

    /// An implicit column block automatically splits its content by a given number of columns.
    /// For details and examples see explicit column block in the [Unimarkup specification](https://github.com/Unimarkup/Specification/blob/main/Frontend/EnclosedBlocks.md#implicit-column-block).
    ImplicitColumn,

    /// This element adds a field name at the start of a text block that is enclosed inside `<>`.
    ///
    /// Example:
    /// ```markdown
    /// [[[<field name>
    ///
    /// Any Unimarkup content.
    ///
    /// ]]]
    /// ```
    FieldBlock,

    /// Every content inside an output block is forwarded as is to the rendered document.
    ///
    /// Example:
    /// ```markdown
    /// <<<
    /// <strong>Some important text</strong>
    /// >>>
    /// ```
    OutputBlock,

    /// Media insert make it possible to insert video and audio files
    /// in addition to images using an extended figure insert syntax.
    ///
    /// For more info, go to Media insert in the [Unimarkup specification](https://github.com/Unimarkup/Specification/blob/main/Frontend/AtomicBlocks.md#media-insert).
    MediaInsert,

    /// It is possible to define macros in Unimarkup. For more information see the Macros section
    /// in the [Unimarkup specification](https://github.com/Unimarkup/Specification/blob/main/Frontend/Macros.md).
    MacroDefinition,

    /// Usage of a defined macro. For more information see the Macros section
    /// in the [Unimarkup specification](https://github.com/Unimarkup/Specification/blob/main/Frontend/Macros.md).
    MacroUsage,

    /// It is possible to define variables in Unimarkup. For more information see the Variables section
    /// in the [Unimarkup specification](https://github.com/Unimarkup/Specification/blob/main/Frontend/Variables.md).
    VariableDefinition,

    /// Usage of a defined variable. For more information see the Variables section
    /// in the [Unimarkup specification](https://github.com/Unimarkup/Specification/blob/main/Frontend/Variables.md).
    VariableUsage,
}

/// Generate implementation of From<_> trait for UnimarkupType for a unimarkup block struct
///
/// ## Usage
///
/// ```ignore
/// from_block_to_type!(Heading, Heading);
///
/// // expands to
///
/// impl From<&Heading> for ElementType {
///     fn from(_: &Heading) -> Self {
///         Self::Heading
///     }
/// }
/// ```
macro_rules! from_block_to_type {
    ($($struct:ty, $variant:ident),*) => {
        $(
            impl From<&$struct> for ElementType {
                fn from(_: &$struct) -> Self {
                    Self::$variant
                }
            }
        )*
    };
}

from_block_to_type!(super::atomic::Heading, Heading);
from_block_to_type!(super::atomic::Paragraph, Paragraph);
from_block_to_type!(super::enclosed::Verbatim, Verbatim);

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test__convert_types__heading() {
        let heading = crate::elements::atomic::Heading::default();

        let um_type = ElementType::from(&heading);

        assert_eq!(um_type, ElementType::Heading);
    }
}
