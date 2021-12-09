use clap::ArgEnum;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, Clone, EnumString, ArgEnum, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
pub enum UnimarkupType {
    #[strum(ascii_case_insensitive)]
    Paragraph,

    #[strum(ascii_case_insensitive)]
    Heading,

    #[strum(ascii_case_insensitive, serialize = "bullet-list")]
    BulletList,

    #[strum(ascii_case_insensitive, serialize = "numbered-list")]
    NumberedList,

    #[strum(ascii_case_insensitive, serialize = "task-list")]
    TaskList,

    #[strum(ascii_case_insensitive, serialize = "definition-list")]
    DefinitionList,

    #[strum(ascii_case_insensitive)]
    Table,

    #[strum(ascii_case_insensitive, serialize = "verbatim-block")]
    VerbatimBlock,

    #[strum(ascii_case_insensitive, serialize = "render-block")]
    RenderBlock,

    #[strum(ascii_case_insensitive, serialize = "math-block")]
    MathBlock,

    #[strum(ascii_case_insensitive, serialize = "figure-insert")]
    FigureInsert,

    #[strum(ascii_case_insensitive, serialize = "verbatim-block-insert")]
    VerbatimBlockInsert,

    #[strum(ascii_case_insensitive, serialize = "render-block-insert")]
    RenderBlockInsert,

    #[strum(ascii_case_insensitive, serialize = "text-block")]
    TextBlock,

    #[strum(ascii_case_insensitive, serialize = "quotation-block")]
    QuotationBlock,

    #[strum(ascii_case_insensitive, serialize = "line-block")]
    LineBlock,

    #[strum(ascii_case_insensitive, serialize = "definition-block")]
    DefinitionBlock,

    #[strum(ascii_case_insensitive, serialize = "explicit-column")]
    ExplicitColumn,

    #[strum(ascii_case_insensitive, serialize = "implicit-column")]
    ImplicitColumn,

    #[strum(ascii_case_insensitive, serialize = "field-block")]
    FieldBlock,

    #[strum(ascii_case_insensitive, serialize = "output-block")]
    OutputBlock,

    #[strum(ascii_case_insensitive, serialize = "media-block-insert")]
    MediaBlockInsert,

    #[strum(ascii_case_insensitive, serialize = "form-block")]
    FormBlock,

    #[strum(ascii_case_insensitive, serialize = "macro-definition")]
    MacroDefinition,

    #[strum(ascii_case_insensitive, serialize = "variable-definition")]
    VariableDefinition,
}
