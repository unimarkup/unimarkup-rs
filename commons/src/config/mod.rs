use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    str::FromStr,
};

use clap::{crate_authors, Args, Parser, ValueEnum};
use serde::{Deserialize, Serialize};

const UNIMARKUP_NAME: &str = "unimarkup";
const ABOUT: &str = "The official compiler for Unimarkup.";
const HELP_TEMPLATE: &str = "{about-section}\nAuthors: {author}";

#[derive(Parser, Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[command(name = UNIMARKUP_NAME, help_template = HELP_TEMPLATE, author = crate_authors!(", "), version, about = ABOUT, long_about = None)]
pub struct Config {
    #[command(flatten)]
    pub preamble: Preamble,
    #[command(flatten)]
    pub merging: MergingConfig,
    pub input: PathBuf,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MergingConfig {
    #[arg(long)]
    pub ignore_preamble: bool,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Preamble {
    #[command(flatten)]
    #[serde(flatten)]
    pub output: Output,
    #[command(flatten)]
    #[serde(flatten)]
    pub metadata: Metadata,
    #[command(flatten)]
    #[serde(flatten)]
    pub cite: Citedata,
    #[command(flatten)]
    #[serde(flatten)]
    pub render: RenderConfig,
    #[command(flatten)]
    #[serde(flatten)]
    pub i18n: I18n,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct I18n {
    #[arg(long, default_value_t = String::from("en-US"))]
    pub lang: String,
    #[arg(long, value_parser = parse_to_hashset::<String>, required = false, default_value = "")]
    pub langs: HashSet<String>,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderConfig {
    #[arg(long, value_parser = parse_to_hashset::<String>, required = false, default_value = "")]
    pub ignore: HashSet<String>,
    #[arg(long, value_parser = parse_parameter, required = false, default_value = "")]
    pub parameter: HashMap<String, String>,
    #[arg(long)]
    pub keep_comments: bool,
    #[arg(long)]
    pub non_strict: bool,
}

// TODO: Instead of PathBufs, file contents should be parsed on deserialization.
// This makes it easier to access the parsed data without creating another config struct.
// It also makes compiling faster for bad inputs, since it would break before parsing starts.
#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Citedata {
    #[arg(long)]
    pub style: Option<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<PathBuf>, required = false, default_value = "")]
    pub references: HashSet<PathBuf>,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    #[arg(long)]
    pub title: Option<String>,
    #[arg(long, value_parser = parse_to_hashset::<String>, required = false, default_value = "")]
    pub authors: HashSet<String>,
    #[arg(long)]
    pub description: Option<String>,
    #[arg(long)]
    pub base: Option<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<PathBuf>, required = false, default_value = "")]
    pub fonts: HashSet<PathBuf>,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Output {
    #[arg(long)]
    pub file: Option<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<OutputFormat>)]
    pub formats: HashSet<OutputFormat>,
    #[command(flatten)]
    #[serde(flatten)]
    pub format_specific: OutputFormatSpecific,
    /// `true` overwrites existing output files
    #[clap(long)]
    pub overwrite: bool,
}

#[derive(
    Default,
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    ValueEnum,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum OutputFormat {
    #[default]
    Html,
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "html" => Ok(OutputFormat::Html),
            o => Err(format!("Bad output format: {}", o)),
        }
    }
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputFormatSpecific {
    #[command(flatten)]
    #[serde(flatten)]
    pub html: HtmlSpecific,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HtmlSpecific {
    #[arg(long, value_parser = parse_to_hashset::<PathBuf>, required = false, default_value = "")]
    pub favicons: HashSet<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<String>, required = false, default_value = "")]
    pub keywords: HashSet<String>,
}

pub fn parse_to_hashset<T>(s: &str) -> Result<HashSet<T>, clap::Error>
where
    T: std::str::FromStr + std::cmp::Eq + std::hash::Hash,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let try_entries: Result<Vec<T>, _> = s.split(',').map(|e| T::from_str(e.trim())).collect();
    let entries = try_entries.map_err(|err| {
        clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            format!("HashSet conversion failed with: {:?}", err),
        )
    });
    Ok(HashSet::from_iter(entries?.into_iter()))
}

pub fn parse_parameter(_s: &str) -> Result<HashMap<String, String>, clap::Error> {
    //TODO: Implement once parameter parser is implemented

    Ok(HashMap::default())
}
