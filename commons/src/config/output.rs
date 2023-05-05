use std::{collections::HashSet, path::PathBuf, str::FromStr};

use clap::{Args, ValueEnum};
use serde::{Deserialize, Serialize};

use super::parse_to_hashset;

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Output {
    pub file: Option<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<OutputFormat>)]
    pub formats: HashSet<OutputFormat>,
    #[command(flatten)]
    #[serde(flatten)]
    pub format_specific: OutputFormatSpecific,
    /// `true` overwrites existing output files
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
    #[arg(long, value_parser = parse_to_hashset::<PathBuf>)]
    pub favicons: HashSet<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<String>)]
    pub keywords: HashSet<String>,
}
