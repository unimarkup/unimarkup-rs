use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use clap::Args;
use logid::set_event_with;
use serde::{Deserialize, Serialize};

use crate::log_id::COMMONS_LOG_ID_MAP;

use super::{log_id::ConfigErrLogId, output::Output, parse_to_hashset, ConfigFns, ReplaceIfNone};

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

impl ConfigFns for Preamble {
    fn merge(&mut self, other: Self) {
        self.output.merge(other.output);
        self.metadata.merge(other.metadata);
        self.cite.merge(other.cite);
        self.render.merge(other.render);
        self.i18n.merge(other.i18n);
    }

    fn validate(&self) -> Result<(), logid::log_id::LogId> {
        self.output.validate()?;
        self.metadata.validate()?;
        self.cite.validate()?;
        self.render.validate()?;
        self.i18n.validate()
    }
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct I18n {
    #[arg(default_value_t = String::from("en-US"))]
    pub lang: String,
    #[arg(long, value_parser = parse_to_hashset::<String>)]
    pub langs: HashSet<String>,
}

impl ConfigFns for I18n {
    fn merge(&mut self, other: Self) {
        self.langs.extend(other.langs.into_iter());
    }

    fn validate(&self) -> Result<(), logid::log_id::LogId> {
        // TODO: make sure strings are valid bcp-47 locales

        Ok(())
    }
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderConfig {
    #[arg(long, value_parser = parse_to_hashset::<String>)]
    pub ignore: HashSet<String>,
    #[arg(long, value_parser = parse_parameter)]
    pub parameter: HashMap<String, String>,
    pub keep_comments: bool,
    pub allow_unsafe: bool,
}

impl ConfigFns for RenderConfig {
    fn merge(&mut self, other: Self) {
        self.ignore.extend(other.ignore.into_iter());
        self.parameter.extend(other.parameter.into_iter());
    }

    fn validate(&self) -> Result<(), logid::log_id::LogId> {
        // TODO: validate ignore and parameter syntax
        Ok(())
    }
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

impl ConfigFns for Citedata {
    fn merge(&mut self, other: Self) {
        self.style.replace_none(other.style);
        self.references.extend(other.references.into_iter());
    }

    fn validate(&self) -> Result<(), logid::log_id::LogId> {
        if let Some(file) = &self.style {
            if !file.exists() {
                return Err(set_event_with!(
                    ConfigErrLogId::InvalidFile,
                    &COMMONS_LOG_ID_MAP,
                    &format!("Citation Style Language file not found: {:?}", file)
                )
                .into());
            }
        }

        for reference in &self.references {
            if !reference.exists() {
                return Err(set_event_with!(
                    ConfigErrLogId::InvalidFile,
                    &COMMONS_LOG_ID_MAP,
                    &format!("Bibliography references file not found: {:?}", reference)
                )
                .into());
            }
        }

        Ok(())
    }
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

impl ConfigFns for Metadata {
    fn merge(&mut self, other: Self) {
        self.title.replace_none(other.title);
        self.authors.extend(other.authors.into_iter());
        self.fonts.extend(other.fonts.into_iter());

        // Note: `base` and `description` must not be merged with sub-configs according to specification.
    }

    fn validate(&self) -> Result<(), logid::log_id::LogId> {
        for font in &self.fonts {
            if !font.exists() {
                return Err(set_event_with!(
                    ConfigErrLogId::InvalidFile,
                    &COMMONS_LOG_ID_MAP,
                    &format!("Font file not found: {:?}", font)
                )
                .into());
            }
        }

        Ok(())
    }
}

pub fn parse_parameter(_s: &str) -> Result<HashMap<String, String>, clap::Error> {
    //TODO: Implement once parameter parser is implemented

    Ok(HashMap::default())
}
