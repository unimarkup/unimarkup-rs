
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
    PDF,
    HTML,
    REVEAL_JS,
    INTERMEDIATE,
}

pub enum HtmlMathmode {
    SVG,
    EMBED,
    CDN,
}

pub enum UmBlockElements {
    PARAGRAPH,
    HEADING,
    BULLET_LIST,
    NUMBERED_LIST,
    TASK_LIST,
    DEFINITION_LIST,
    TABLE,
    VERBATIM_BLOCK,
    RENDER_BLOCK,
    MATH_BLOCK,
    FIGURE_INSERT,
    VERBATIM_BLOCK_INSERT,
    RENDER_BLOCK_INSERT,
    TEXT_BLOCK,
    QUOTATION_BLOCK,
    LINE_BLOCK,
    DEFINITION_BLOCK,
    EXPLICIT_COLUMN,
    IMPLICIT_COLUMN,
    FIELD_BLOCK,
    OUTPUT_BLOCK,
    MEDIA_BLOCK_INSERT,
    FORM_BLOCK,
    MACROS,
    VARIABLES,
}


