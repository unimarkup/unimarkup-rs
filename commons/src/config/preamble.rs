use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use clap::Args;
use serde::{Deserialize, Serialize};

use super::{output::Output, parse_to_hashset};

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
    #[arg(default_value_t = String::from("en-US"))]
    pub lang: String,
    #[arg(long, value_parser = parse_to_hashset::<String>)]
    pub langs: HashSet<String>,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderConfig {
    #[arg(long, value_parser = parse_to_hashset::<String>)]
    pub ignore: HashSet<String>,
    #[arg(long, value_parser = parse_parameter)]
    pub parameter: HashMap<String, String>,
    pub keep_comments: bool,
    pub non_strict: bool,
}

// TODO: Instead of PathBufs, file contents should be parsed on deserialization.
// This makes it easier to access the parsed data without creating another config struct.
// It also makes compiling faster for bad inputs, since it would break before parsing starts.
#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Citedata {
    pub style: Option<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<PathBuf>)]
    pub references: HashSet<PathBuf>,
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    pub title: Option<String>,
    #[arg(long, value_parser = parse_to_hashset::<String>)]
    pub authors: HashSet<String>,
    pub description: Option<String>,
    pub base: Option<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<PathBuf>)]
    pub fonts: HashSet<PathBuf>,
}

pub fn parse_parameter(_s: &str) -> Result<HashMap<String, String>, clap::Error> {
    //TODO: Implement once parameter parser is implemented

    Ok(HashMap::default())
}
