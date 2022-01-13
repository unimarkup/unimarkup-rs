//! Structs and functions for parsing and generating the Unimarkup configuration.

use std::path::PathBuf;

use clap::{crate_version, ArgEnum, Parser};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

use crate::um_elements::types::UnimarkupType;

const UNIMARKUP_NAME: &str = "Unimarkup";

/// Config contains the possible options for the Unimarkup configuration.
#[derive(Debug, PartialEq, Clone, Parser, Default, Serialize, Deserialize)]
#[clap(name = UNIMARKUP_NAME, version = crate_version!())]
pub struct Config {
    /// The filename of the Unimarkup file that is used as root for rendering.
    #[clap(
        index = 1,
        value_name = "UM-FILE",
        required = true,
        takes_value = true,
        parse(from_os_str)
    )]
    #[serde(skip)]
    pub um_file: PathBuf,

    /// The filename without filetype to be used for output filenames. If a path is part of the filename, output files are saved at the given path.
    #[clap(
        index = 2,
        value_name = "OUTPUT-FILE",
        takes_value = true,
        parse(from_os_str)
    )]
    #[serde(alias = "OUTPUT-FILE")]
    #[serde(default)]
    pub out_file: Option<PathBuf>,

    /// Set output formats the Unimarkup document should be rendered to. Set outputs are also treated as flags inside the Unimarkup document.
    #[clap(
        name = "output-formats",
        display_order = 1,
        long = "output-formats",
        alias = "formats",
        takes_value = true,
        use_delimiter = true,
        arg_enum
    )]
    #[serde(alias = "output-formats")]
    #[serde(alias = "formats")]
    #[serde(default)]
    pub out_formats: Option<Vec<OutputFormat>>,

    /// Set paths that are searched for relative file and image inserts.
    #[clap(
        display_order = 2,
        long = "insert-paths",
        takes_value = true,
        use_delimiter = true,
        parse(from_os_str)
    )]
    #[serde(alias = "insert-paths")]
    #[serde(default)]
    pub insert_paths: Option<Vec<PathBuf>>,

    /// Set the path to a directory that is used for default preamble and theme settings.
    /// The intermediate form of rendered documents will also be stored at this path.
    #[clap(
        display_order = 3,
        long = "dot-unimarkup",
        alias = "config",
        takes_value = true,
        env = "UNIMARKUP_CONFIG",
        parse(from_os_str)
    )]
    #[serde(alias = "dot-unimarkup")]
    #[serde(alias = "config")]
    #[serde(default)]
    pub dot_unimarkup: Option<PathBuf>,

    /// Set a Unimarkup theme file to be used for rendering.
    #[clap(
        display_order = 5,
        short = 't',
        long = "theme",
        takes_value = true,
        parse(from_os_str)
    )]
    #[serde(alias = "theme")]
    #[serde(alias = "t")]
    #[serde(default)]
    pub theme: Option<PathBuf>,

    /// Set flags that will be set for rendering.
    #[clap(
        display_order = 6,
        short = 'f',
        long = "flags",
        takes_value = true,
        use_delimiter = true
    )]
    #[serde(alias = "flags")]
    #[serde(alias = "f")]
    #[serde(default)]
    pub flags: Option<Vec<String>>,

    /// Explicitly set Unimarkup block elements that can be used inside the given Unimarkup document.
    /// If this option is set, all Unimarkup elements that are not given are disabled.
    #[clap(
        display_order = 7,
        long = "enable-elements",
        takes_value = true,
        use_delimiter = true,
        arg_enum
    )]
    #[serde(alias = "enable-elements")]
    #[serde(default)]
    pub enable_elements: Option<Vec<UnimarkupType>>,

    /// Explicitly set Unimarkup block elements that can NOT be used inside the given Unimarkup document.
    /// If this option is set, all Unimarkup elements that are not given are enabled.
    #[clap(
        display_order = 8,
        long = "disable-elements",
        takes_value = true,
        use_delimiter = true,
        arg_enum
    )]
    #[serde(alias = "disable-elements")]
    #[serde(default)]
    pub disable_elements: Option<Vec<UnimarkupType>>,

    /// Set citation style sheet that is used to process referenced literature
    #[clap(
        display_order = 30,
        long = "citation-style",
        alias = "csl",
        takes_value = true,
        requires = "references",
        parse(from_os_str)
    )]
    #[serde(alias = "citation-style")]
    #[serde(alias = "csl")]
    #[serde(default)]
    pub citation_style: Option<PathBuf>,

    /// Set one or more reference files in bibtex or JSON format to use them with literature referencing.
    #[clap(
        display_order = 31,
        long = "references",
        alias = "refs",
        takes_value = true,
        use_delimiter = true,
        requires = "citation-style",
        parse(from_os_str)
    )]
    #[serde(alias = "references")]
    #[serde(alias = "refs")]
    #[serde(default)]
    pub references: Option<Vec<PathBuf>>,

    /// Set ttf or woff fonts to be able to use them for rendering
    #[clap(
        display_order = 10,
        long = "fonts",
        alias = "ttf",
        alias = "woff",
        takes_value = true,
        use_delimiter = true,
        parse(from_os_str)
    )]
    #[serde(alias = "fonts")]
    #[serde(alias = "ttf")]
    #[serde(alias = "woff")]
    #[serde(default)]
    pub fonts: Option<Vec<PathBuf>>,

    /// Overwrites files set with `out-file` if already existing.
    #[clap(
        display_order = 1,
        short = 'w',
        long = "overwrite-out-files",
        takes_value = false
    )]
    #[serde(alias = "overwrite-out-files")]
    #[serde(alias = "w")]
    #[serde(default)]
    pub overwrite_out_files: bool,

    /// Deletes all previously rendered documents stored inside the UNIMARKUP_CONFIG path.
    #[clap(display_order = 2, short = 'c', long = "clean", takes_value = false)]
    #[serde(alias = "clean")]
    #[serde(alias = "c")]
    #[serde(default)]
    pub clean: bool,

    /// Ignores all previously rendered documents stored inside the UNIMARKUP_CONFIG path and renders the given Unimarkup file.
    #[clap(display_order = 3, short = 'r', long = "rebuild", takes_value = false)]
    #[serde(alias = "rebuild")]
    #[serde(alias = "r")]
    #[serde(default)]
    pub rebuild: bool,

    /// Set if preamble of given Unimarkup file is replaced with the given arguments.
    /// If not set, given arguments overwrite the corresponding preamble settings, but other settings are still used.
    #[clap(
        display_order = 20,
        long = "replace-preamble",
        requires = "output-formats",
        takes_value = false
    )]
    #[serde(skip)]
    pub replace_preamble: bool,

    /// This prefix will be set before inserts in the rendered document to inserts that use relative paths.
    /// Note: During rendering, the original relative path is taken.
    #[clap(
        display_order = 20,
        long = "relative-insert-prefix",
        alias = "insert-prefix",
        takes_value = true,
        parse(from_os_str)
    )]
    #[serde(alias = "relative-insert-prefix")]
    #[serde(alias = "insert-prefix")]
    #[serde(default)]
    pub relative_insert_prefix: Option<PathBuf>,

    /// Set a template html file with `{{ head }}` set inside the `head` element and `{{ body }}` set inside the body element.
    /// Styling, fonts and scripts will be inserted at `{{ head }}` and the rendered Unimarkup content is placed inside `{{ body }}`.
    /// Optionally, `{{ toc }}` can be set to get the table of contents (Note: This will not remove a rendered table of contents inside the rendered Unimarkup content if present).
    #[clap(
        display_order = 40,
        long = "html-template",
        takes_value = true,
        parse(from_os_str)
    )]
    #[serde(alias = "html-template")]
    #[serde(default)]
    pub html_template: Option<PathBuf>,

    /// Set the mathmode of MathJax to be used for rendered html documents.
    #[clap(
        display_order = 41,
        long = "html-mathmode",
        takes_value = true,
        arg_enum
    )]
    #[serde(alias = "html-mathmode")]
    #[serde(default)]
    pub html_mathmode: Option<HtmlMathmode>,

    /// Set if svgs should be embedded into html instead of inserted as regular images.
    #[clap(display_order = 40, long = "html-embed-svg", takes_value = false)]
    #[serde(alias = "html-embed-svg")]
    #[serde(default)]
    pub html_embed_svg: bool,
}

/// Possible output formats for a Unimarkup file
#[derive(
    Debug, PartialEq, Clone, EnumString, ArgEnum, strum_macros::Display, Serialize, Deserialize,
)]
pub enum OutputFormat {
    /// PDF output format
    #[strum(ascii_case_insensitive)]
    Pdf,
    /// HTML output format
    #[strum(ascii_case_insensitive)]
    Html,
    #[strum(ascii_case_insensitive, serialize = "reveal-js")]
    #[clap(alias = "revealjs")]
    /// [revealJs] output format.
    ///
    /// A presentation framework using HTML and Javascript
    ///
    /// [revealJs]: https://revealjs.com/
    RevealJs,
    #[strum(ascii_case_insensitive)]
    /// Intermediate representation of the Unimarkup document.
    Intermediate,
}

/// Possible modes for rendering math formulas in HTML
#[derive(
    Debug, PartialEq, Clone, EnumString, ArgEnum, strum_macros::Display, Serialize, Deserialize,
)]
pub enum HtmlMathmode {
    /// Render math as SVG
    #[strum(ascii_case_insensitive)]
    Svg,
    /// Embed MathJax
    #[strum(ascii_case_insensitive)]
    Embed,
    /// Use CDN (Content Delivery Network) for MathJax (requires online connection to view math formulas in the output HTML)
    #[strum(ascii_case_insensitive)]
    Cdn,
}
