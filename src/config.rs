use strum_macros::EnumString;

#[derive(Debug, PartialEq, Clone)]
pub struct Config {
    pub um_file: String,
    pub out_file: String,
    pub insert_paths: Vec<String>,
    pub dot_unimarkup: String,
    pub theme: String,
    pub flags: Vec<String>,
    pub enable_elements: Vec<UmBlockElements>,
    pub disable_elements: Vec<UmBlockElements>,
    pub citation_style: String,
    pub references: Vec<String>,
    pub fonts: Vec<String>,
    pub overwrite_existing: bool,
    pub clean: bool,
    pub rebuild: bool,
    pub replace_preamble: bool,
    pub outputs: Vec<OutputConfig>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OutputConfig {
    pub out_file: String,
    pub out_type: OutputFormat,
    pub theme: String,
    pub flags: Vec<String>,
    pub relative_insert_prefix: String,
    pub html_template: String, 
    pub html_mathmode: HtmlMathmode,
    pub html_embed_svg: bool,
}

#[derive(Debug, PartialEq, Clone, EnumString)]
pub enum OutputFormat {
    #[strum(ascii_case_insensitive)]
    Pdf,
    #[strum(ascii_case_insensitive)]
    Html,
    #[strum(ascii_case_insensitive, serialize = "reveal-js")]
    RevealJs,
    #[strum(ascii_case_insensitive)]
    Intermediate,
}

#[derive(Debug, PartialEq, Clone, EnumString)]
pub enum HtmlMathmode {
    #[strum(ascii_case_insensitive)]
    Svg,
    #[strum(ascii_case_insensitive)]
    Embed,
    #[strum(ascii_case_insensitive)]
    Cdn,
}

#[derive(Debug, PartialEq, Clone, EnumString)]
pub enum UmBlockElements {
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
