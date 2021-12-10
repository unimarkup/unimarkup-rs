use clap::ArgEnum;
use strum_macros::EnumString;

pub const DELIMITER: char = '-';

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
