use std::{path::PathBuf, process};
use log::error;

use clap::{ArgEnum, Parser, crate_version};
use strum_macros::EnumString;

const UNIMARKUP_NAME: &str = "Unimarkup";

#[derive(Debug, PartialEq, Clone, Parser)]
#[clap(name = UNIMARKUP_NAME, version = crate_version!())]
pub struct Config {
    /// The filename of the Unimarkup file that is used as root for rendering.
    #[clap(index = 1, value_name = "UM-FILE", required = true, takes_value = true, parse(from_os_str))]
    pub um_file: PathBuf,

    /// The filename without filetype to be used for output filenames. If a path is part of the filename, output files are saved at the given path.
    #[clap(index = 2, value_name = "OUTPUT-FILE", takes_value = true, parse(from_os_str))]
    pub out_file: Option<PathBuf>,

    /// Set outputs for the Unimarkup document. The output options must be set after -O and enclosed in "" like `-O="myOutputConfig --formats=pdf flags=custom_flag"`.
    /// The first argument must be the name of the output config. This is used to compare output configurations between cli and preamble.
    /// The following output options are available:
    /// 
    /// USAGE:
    ///    <OutputConfig Name> --output-formats <OUT_FORMATS>... [--] [OUTPUT-FILE]
    ///
    /// ARGS:
    ///    <OUTPUT-FILE>    The filename without filetype to be used for output filenames. If a path is
    ///                     part of the filename, output files are saved at the given path [default: ]
    ///
    /// OPTIONS:
    ///        --output-formats <OUT_FORMATS>...
    ///            Set output formats the Unimarkup document should be rendered to. Set outputs are also
    ///            treated as flags inside the Unimarkup document [possible values: pdf, html, reveal-js, intermediate]
    ///
    /// Same additional options as for the global configuration
    /// 
    // TODO: #[doc = stringify!(OutputConfig::parse_from(vec![UNIMARKUP_NAME, "--help"]))]
    #[clap(display_order = 1, short = 'O', long = "output", parse(try_from_str = parse_output_config), multiple_occurrences(true), takes_value = true)]
    pub outputs: Option<Vec<(String, OutputConfig)>>,

    /// Set paths that are searched for relative file and image inserts.
    #[clap(display_order = 2, long = "insert-paths", takes_value = true, use_delimiter = true, parse(from_os_str))]
    pub insert_paths: Option<Vec<PathBuf>>,

    /// Set the path to a directory that is used for default preamble and theme settings.
    /// The intermediate form of rendered documents will also be stored at this path.
    #[clap(display_order = 3, long = "dot-unimarkup", alias = "config", takes_value = true, env = "UNIMARKUP_CONFIG", parse(from_os_str))]
    pub dot_unimarkup: Option<PathBuf>,

    /// Set a Unimarkup theme file to be used for rendering.
    #[clap(display_order = 5, short = 't', long = "theme", takes_value = true, parse(from_os_str))]
    pub theme: Option<PathBuf>,

    /// Set flags that will be set for rendering.
    #[clap(display_order = 6, short = 'f', long = "flags", takes_value = true, use_delimiter = true)]
    pub flags: Option<Vec<String>>,

    /// Explicitly set Unimarkup block elements that can be used inside the given Unimarkup document.
    /// If this option is set, all Unimarkup elements that are not given are disabled.
    #[clap(display_order = 7, long = "enable-elements", takes_value = true, use_delimiter = true, arg_enum)]
    pub enable_elements: Option<Vec<UmBlockElements>>,

    /// Explicitly set Unimarkup block elements that can NOT be used inside the given Unimarkup document.
    /// If this option is set, all Unimarkup elements that are not given are enabled.
    #[clap(display_order = 8, long = "disable-elements", takes_value = true, use_delimiter = true, arg_enum)]
    pub disable_elements: Option<Vec<UmBlockElements>>,

    /// Set citation style sheet that is used to process referenced literature
    #[clap(display_order = 30, long = "citation-style", alias = "csl", takes_value = true, requires = "references", parse(from_os_str))]
    pub citation_style: Option<PathBuf>,

    /// Set one or more reference files in bibtex or JSON format to use them with literature referencing.
    #[clap(display_order = 31, long = "references", alias = "refs", takes_value = true, use_delimiter = true, requires = "citation-style", parse(from_os_str))]
    pub references: Option<Vec<PathBuf>>,

    /// Set ttf or woff fonts to be able to use them for rendering
    #[clap(display_order = 10, long = "fonts", alias = "ttf", alias = "woff", takes_value = true, use_delimiter = true, parse(from_os_str))]
    pub fonts: Option<Vec<PathBuf>>,

    /// Overwrites files with set `filename` if already existing.
    #[clap(display_order = 1, short = 'w', long = "overwrite-existing")]
    pub overwrite_existing: Option<bool>,

    /// Deletes all previously rendered documents stored inside the UNIMARKUP_CONFIG path.
    #[clap(display_order = 2, short = 'c', long = "clean")]
    pub clean: Option<bool>,

    /// Ignores all previously rendered documents stored inside the UNIMARKUP_CONFIG path and renders the given Unimarkup file.
    #[clap(display_order = 3, short = 'r', long = "rebuild")]
    pub rebuild: Option<bool>,

    /// Set if preamble of given Unimarkup file is replaced with the given arguments.
    /// If not set, given arguments overwrite the corresponding preamble settings, but other settings are still used.
    /// 
    /// Note: Only the preamble part of an output config with the same name is replaced if this flag is only set inside an output config   
    #[clap(display_order = 20, long = "replace-preamble", requires = "outputs")]
    pub replace_preamble: Option<bool>,

    /// This prefix will be set before inserts in the rendered document to inserts that use relative paths.
    /// Note: During rendering, the original relative path is taken.
    #[clap(display_order = 20, long = "relative-insert-prefix", alias = "insert-prefix", takes_value = true, parse(from_os_str))]
    pub relative_insert_prefix: Option<PathBuf>,

    /// Set a template html file with `{{ head }}` set inside the `head` element and `{{ body }}` set inside the body element.
    /// Styling, fonts and scripts will be inserted at `{{ head }}` and the rendered Unimarkup content is placed inside `{{ body }}`.
    /// Optionally, `{{ toc }}` can be set to get the table of contents (Note: This will not remove a rendered table of contents inside the rendered Unimarkup content if present).
    #[clap(display_order = 40, long = "html-template", takes_value = true, parse(from_os_str))]
    pub html_template: Option<PathBuf>, 

    /// Set the mathmode of MathJax to be used for rendered html documents.
    #[clap(display_order = 41, long = "html-mathmode", takes_value = true, arg_enum)]
    pub html_mathmode: Option<HtmlMathmode>,

    /// Set if svgs should be embedded into html instead of inserted as regular images.
    #[clap(display_order = 40, long = "html-embed-svg")]
    pub html_embed_svg: Option<bool>,
}

/// Parse a single output config 
fn parse_output_config(s: &str) -> Result<(String, OutputConfig), clap::Error>
{
    let args : Vec<String> = shlex::Shlex::new(s).collect(); // s.trim().split_whitespace().collect();
    //let mut params = vec![UNIMARKUP_NAME];
    //params.extend(args);
    if args.is_empty() {
        error!("No output config name was given! Every output config must start with a unique name!");
        process::exit(1)
    }

    let cfg_name = args[0].clone();
    let out_config = OutputConfig::try_parse_from(args)?;
    Ok((cfg_name, out_config))
}

#[derive(Debug, PartialEq, Clone, Parser)]
pub struct OutputConfig {
    /// The filename without filetype to be used for output filenames. If a path is part of the filename, output files are saved at the given path.
    #[clap(index = 1, value_name = "OUTPUT-FILE", takes_value = true, parse(from_os_str))]
    pub out_file: Option<PathBuf>,

    /// Set output formats the Unimarkup document should be rendered to. Set outputs are also treated as flags inside the Unimarkup document.
    #[clap(name = "output-formats",display_order = 1, long = "output-formats", alias = "formats", takes_value = true, use_delimiter = true, required = true, arg_enum)]
    pub out_formats: Vec<OutputFormat>,

    /// Set paths that are searched for relative file and image inserts.
    #[clap(display_order = 2, long = "insert-paths", takes_value = true, use_delimiter = true, parse(from_os_str))]
    pub insert_paths: Option<Vec<PathBuf>>,

    /// Set the path to a directory that is used for default preamble and theme settings.
    /// The intermediate form of rendered documents will also be stored at this path.
    #[clap(display_order = 3, long = "dot-unimarkup", alias = "config", takes_value = true, env = "UNIMARKUP_CONFIG", parse(from_os_str))]
    pub dot_unimarkup: Option<PathBuf>,

    /// Set a Unimarkup theme file to be used for rendering.
    #[clap(display_order = 5, short = 't', long = "theme", takes_value = true, parse(from_os_str))]
    pub theme: Option<PathBuf>,

    /// Set flags that will be set for rendering.
    #[clap(display_order = 6, short = 'f', long = "flags", takes_value = true, use_delimiter = true)]
    pub flags: Option<Vec<String>>,

    /// Explicitly set Unimarkup block elements that can be used inside the given Unimarkup document.
    /// If this option is set, all Unimarkup elements that are not given are disabled.
    #[clap(display_order = 7, long = "enable-elements", takes_value = true, use_delimiter = true, arg_enum)]
    pub enable_elements: Option<Vec<UmBlockElements>>,

    /// Explicitly set Unimarkup block elements that can NOT be used inside the given Unimarkup document.
    /// If this option is set, all Unimarkup elements that are not given are enabled.
    #[clap(display_order = 8, long = "disable-elements", takes_value = true, use_delimiter = true, arg_enum)]
    pub disable_elements: Option<Vec<UmBlockElements>>,

    /// Set citation style sheet that is used to process referenced literature
    #[clap(display_order = 30, long = "citation-style", alias = "csl", takes_value = true, requires = "references", parse(from_os_str))]
    pub citation_style: Option<PathBuf>,

    /// Set one or more reference files in bibtex or JSON format to use them with literature referencing.
    #[clap(display_order = 31, long = "references", alias = "refs", takes_value = true, use_delimiter = true, requires = "citation-style", parse(from_os_str))]
    pub references: Option<Vec<PathBuf>>,

    /// Set ttf or woff fonts to be able to use them for rendering
    #[clap(display_order = 10, long = "fonts", alias = "ttf", alias = "woff", takes_value = true, use_delimiter = true, parse(from_os_str))]
    pub fonts: Option<Vec<PathBuf>>,

    /// Overwrites files with set `filename` if already existing.
    #[clap(display_order = 1, short = 'w', long = "overwrite-existing")]
    pub overwrite_existing: Option<bool>,

    /// Deletes all previously rendered documents stored inside the UNIMARKUP_CONFIG path.
    #[clap(display_order = 2, short = 'c', long = "clean")]
    pub clean: Option<bool>,

    /// Ignores all previously rendered documents stored inside the UNIMARKUP_CONFIG path and renders the given Unimarkup file.
    #[clap(display_order = 3, short = 'r', long = "rebuild")]
    pub rebuild: Option<bool>,

    /// Set if preamble of given Unimarkup file is replaced with the given arguments.
    /// If not set, given arguments overwrite the corresponding preamble settings, but other settings are still used.
    /// 
    /// Note: Only the preamble part of an output config with the same name is replaced if this flag is only set inside an output config   
    #[clap(display_order = 20, long = "replace-preamble", requires = "output-formats")]
    pub replace_preamble: Option<bool>,

    /// This prefix will be set before inserts in the rendered document to inserts that use relative paths.
    /// Note: During rendering, the original relative path is taken.
    #[clap(display_order = 20, long = "relative-insert-prefix", alias = "insert-prefix", takes_value = true, parse(from_os_str))]
    pub relative_insert_prefix: Option<PathBuf>,

    /// Set a template html file with `{{ head }}` set inside the `head` element and `{{ body }}` set inside the body element.
    /// Styling, fonts and scripts will be inserted at `{{ head }}` and the rendered Unimarkup content is placed inside `{{ body }}`.
    /// Optionally, `{{ toc }}` can be set to get the table of contents (Note: This will not remove a rendered table of contents inside the rendered Unimarkup content if present).
    #[clap(display_order = 40, long = "html-template", takes_value = true, parse(from_os_str))]
    pub html_template: Option<PathBuf>, 

    /// Set the mathmode of MathJax to be used for rendered html documents.
    #[clap(display_order = 41, long = "html-mathmode", takes_value = true, arg_enum)]
    pub html_mathmode: Option<HtmlMathmode>,

    /// Set if svgs should be embedded into html instead of inserted as regular images.
    #[clap(display_order = 40, long = "html-embed-svg")]
    pub html_embed_svg: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, EnumString, ArgEnum)]
pub enum OutputFormat {
    #[strum(ascii_case_insensitive)]
    Pdf,
    #[strum(ascii_case_insensitive)]
    Html,
    #[strum(ascii_case_insensitive, serialize = "reveal-js")]
    #[clap(alias = "revealjs")]
    RevealJs,
    #[strum(ascii_case_insensitive)]
    Intermediate,
}

#[derive(Debug, PartialEq, Clone, EnumString, ArgEnum)]
pub enum HtmlMathmode {
    #[strum(ascii_case_insensitive)]
    Svg,
    #[strum(ascii_case_insensitive)]
    Embed,
    #[strum(ascii_case_insensitive)]
    Cdn,
}

#[derive(Debug, PartialEq, Clone, EnumString, ArgEnum)]
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
