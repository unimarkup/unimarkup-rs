use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Config {
    pub preamble: Preamble,
    pub merging: MergingConfig,
    pub input: PathBuf,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MergingConfig {
    pub ignore_preamble: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Preamble {
    pub output: Output,
    pub metadata: Metadata,
    pub cite: Citedata,
    pub render: RenderConfig,
    pub i18n: I18n,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct I18n {
    pub lang: Option<String>,
    pub langs: HashSet<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RenderConfig {
    /// K = element/attribute name, V = `true` to ignore
    pub ignore: HashMap<String, bool>,
    pub parameter: HashMap<String, String>,
    pub keep_comments: bool,
    pub non_strict: bool,
}

// TODO: Instead of PathBufs, file contents should be parsed on deserialization.
// This makes it easier to access the parsed data without creating another config struct.
// It also makes compiling faster for bad inputs, since it would break before parsing starts.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Citedata {
    pub style: Option<PathBuf>,
    pub references: HashSet<PathBuf>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub title: Option<String>,
    pub authors: HashSet<String>,
    pub description: Option<String>,
    pub base: Option<PathBuf>,
    pub fonts: HashSet<PathBuf>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Output {
    pub file: Option<PathBuf>,
    pub formats: HashSet<OutputFormat>,
    pub format_specific: OutputFormatSpecific,
    /// `true` overwrites existing output files
    pub overwrite: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OutputFormat {
    #[default]
    Html,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct OutputFormatSpecific {
    pub html: HtmlSpecific,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct HtmlSpecific {
    pub favicons: HashSet<PathBuf>,
    pub keywords: HashSet<String>,
}
