
pub struct Config {
    um_file: String,
    out_file: String,
    insert_paths: Vec<String>,
    dot_unimarkup: String,
    theme: String,
    flags: Vec<String>,
    enable_elements: Vec<UmBlockElements>,
    disable_elements: Vec<UmBlockElements>,
    citation_style: String,
    references: String,
    fonts: Vec<String>,
    overwrite_existing: bool,
    clean: bool,
    rebuild: bool,
    replace_preamble: bool,
    outputs: Vec<OutputConfig>,
}

pub struct OutputConfig {
    out_file: String,
    out_type: OutputFormat,
    theme: String,
    flags: Vec<String>,
    relative_insert_prefix: String,
    html_template: String, 
    html_mathmode: HtmlMathmode,
    html_embed_svg: bool,
}

pub enum OutputFormat {
    Pdf,
    Html,
    RevealJs,
    Intermediate,
}

pub enum HtmlMathmode {
    Svg,
    Embed,
    Cdn,
}

pub enum UmBlockElements {
    Paragraph,
    Heading,
    BulletList,
    NumberedList,
    TaskList,
    DefinitionList,
    TABLE,
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
