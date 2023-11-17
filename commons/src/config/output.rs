use std::{collections::HashSet, path::PathBuf, str::FromStr};

use clap::{Args, ValueEnum};
use logid::err;
use serde::{Deserialize, Serialize};

use super::{log_id::ConfigErr, parse_to_hashset, ConfigFns, ReplaceIfNone};

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Output {
    #[arg(long = "output-file")]
    pub file: Option<PathBuf>,
    /// Defines the output format to render to.
    /// If this option is not set, the input is rendered to all supported formats.
    ///
    /// **Supported formats:** `html`
    #[arg(long, alias = "output-formats", value_parser = parse_to_hashset::<OutputFormatKind>, required = false, default_value = "umi")]
    pub formats: HashSet<OutputFormatKind>,
    /// `true` overwrites existing output files
    #[arg(long, alias = "overwrite-out-files")]
    pub overwrite: bool,
}

impl ConfigFns for Output {
    fn merge(&mut self, other: Self) {
        self.file.replace_none(other.file);
        self.formats.extend(other.formats.iter());
    }

    fn validate(&self) -> Result<(), ConfigErr> {
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
                        format!(
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
    Umi,
}

impl OutputFormatKind {
    /// Returns the associated file extension for an output format.
    pub fn extension(&self) -> &str {
        match self {
            OutputFormatKind::Html => "html",
            OutputFormatKind::Umi => "umi",
        }
    }
}

impl FromStr for OutputFormatKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "html" => Ok(OutputFormatKind::Html),
            "umi" => Ok(OutputFormatKind::Umi),
            o => Err(format!("Bad output format: {}", o)),
        }
    }
}
