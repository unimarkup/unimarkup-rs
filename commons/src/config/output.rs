use std::{collections::HashSet, path::PathBuf, str::FromStr};

use clap::{Args, ValueEnum};
use logid::err;
use serde::{Deserialize, Serialize};

use super::{log_id::ConfigErr, parse_to_hashset, ConfigFns, ReplaceIfNone};

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Output {
    #[arg(long = "output-file")]
    pub file: Option<PathBuf>,
    #[arg(long, alias = "output-formats", value_parser = parse_to_hashset::<OutputFormatKind>)]
    pub formats: HashSet<OutputFormatKind>,
    #[command(flatten)]
    #[serde(flatten)]
    pub format_specific: OutputFormatSpecific,
    /// `true` overwrites existing output files
    #[arg(long, alias = "overwrite-out-files")]
    pub overwrite: bool,
}

impl ConfigFns for Output {
    fn merge(&mut self, other: Self) {
        self.file.replace_none(other.file);
        self.formats.extend(other.formats.iter());
        self.format_specific.merge(other.format_specific);
    }

    fn validate(&self) -> Result<(), ConfigErr> {
        self.format_specific.validate()?;

        if self.formats.is_empty() {
            return err!(ConfigErr::InvalidConfig, "No output format was set.");
        }

        if let (Some(ref file), false) = (&self.file, self.overwrite) {
            let mut filepath = file.clone();

            for format in &self.formats {
                filepath.set_extension(format.extension());

                if filepath.exists() {
                    return err!(
                        ConfigErr::InvalidConfig,
                        &format!(
                            "Output file '{:?}' already exists, but `overwrite` was not set.",
                            filepath
                        )
                    );
                }
            }
        }
        Ok(())
    }
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
pub enum OutputFormatKind {
    #[default]
    Html,
}

impl OutputFormatKind {
    /// Returns the associated file extension for an output format.
    pub fn extension(&self) -> &str {
        match self {
            OutputFormatKind::Html => "html",
        }
    }
}

impl FromStr for OutputFormatKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "html" => Ok(OutputFormatKind::Html),
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

impl ConfigFns for OutputFormatSpecific {
    fn merge(&mut self, other: Self) {
        self.html.merge(other.html);
    }

    fn validate(&self) -> Result<(), ConfigErr> {
        self.html.validate()
    }
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HtmlSpecific {
    #[arg(long, value_parser = parse_to_hashset::<PathBuf>, required = false, default_value = "")]
    pub favicons: HashSet<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<String>, required = false, default_value = "")]
    pub keywords: HashSet<String>,
}

impl ConfigFns for HtmlSpecific {
    fn merge(&mut self, other: Self) {
        self.favicons.extend(other.favicons.into_iter());
        self.keywords.extend(other.keywords.into_iter());
    }

    fn validate(&self) -> Result<(), ConfigErr> {
        for fav in &self.favicons {
            if !fav.exists() {
                return err!(
                    ConfigErr::InvalidFile,
                    &format!("Favicon file not found: {:?}", fav)
                );
            }
        }

        Ok(())
    }
}
