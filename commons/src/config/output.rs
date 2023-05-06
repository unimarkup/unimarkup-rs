use std::{collections::HashSet, path::PathBuf, str::FromStr};

use clap::{Args, ValueEnum};
use logid::set_event_with;
use serde::{Deserialize, Serialize};

use crate::log_id::COMMONS_LOG_ID_MAP;

use super::{log_id::ConfigErrLogId, parse_to_hashset, ConfigFns, ReplaceIfNone};

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

impl ConfigFns for Output {
    fn merge(&mut self, other: Self) {
        self.file.replace_none(other.file);
        self.formats.extend(other.formats.iter());
        self.format_specific.merge(other.format_specific);
    }

    fn validate(&self) -> Result<(), logid::log_id::LogId> {
        self.format_specific.validate()?;

        if self.formats.is_empty() {
            return Err(set_event_with!(
                ConfigErrLogId::InvalidConfig,
                &COMMONS_LOG_ID_MAP,
                "No output format was set."
            )
            .into());
        }

        if let Some(ref file) = self.file {
            let mut filepath = file.clone();
            for format in &self.formats {
                filepath.set_extension(format.extension());

                if filepath.exists() && !self.overwrite {
                    return Err(set_event_with!(
                        ConfigErrLogId::InvalidConfig,
                        &COMMONS_LOG_ID_MAP,
                        &format!(
                            "Output file '{:?}' already exists, but `overwrite` was not set.",
                            filepath
                        )
                    )
                    .into());
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
pub enum OutputFormat {
    #[default]
    Html,
}

impl OutputFormat {
    /// Returns the associated file extension for an output format.
    pub fn extension(&self) -> &str {
        match self {
            OutputFormat::Html => "html",
        }
    }
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

impl ConfigFns for OutputFormatSpecific {
    fn merge(&mut self, other: Self) {
        self.html.merge(other.html);
    }

    fn validate(&self) -> Result<(), logid::log_id::LogId> {
        self.html.validate()
    }
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HtmlSpecific {
    #[arg(long, value_parser = parse_to_hashset::<PathBuf>)]
    pub favicons: HashSet<PathBuf>,
    #[arg(long, value_parser = parse_to_hashset::<String>)]
    pub keywords: HashSet<String>,
}

impl ConfigFns for HtmlSpecific {
    fn merge(&mut self, other: Self) {
        self.favicons.extend(other.favicons.into_iter());
        self.keywords.extend(other.keywords.into_iter());
    }

    fn validate(&self) -> Result<(), logid::log_id::LogId> {
        for fav in &self.favicons {
            if !fav.exists() {
                return Err(set_event_with!(
                    ConfigErrLogId::InvalidFile,
                    &COMMONS_LOG_ID_MAP,
                    &format!("Favicon file not found: {:?}", fav)
                )
                .into());
            }
        }

        Ok(())
    }
}
