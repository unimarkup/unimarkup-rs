use std::{collections::HashSet, path::PathBuf};

use clap::{Args, Parser};
use logid::{log_id::LogId, set_event_with};
use serde::{Deserialize, Serialize};

use crate::log_id::COMMONS_LOG_ID_MAP;

use self::{log_id::ConfigErrLogId, preamble::Preamble};

pub mod log_id;
pub mod output;
pub mod preamble;

const UNIMARKUP_NAME: &str = "unimarkup";

/// Trait defining functions every configuration struct must implement.
pub trait ConfigFns {
    /// Merges another configuration struct into **self**.
    /// Merging is done according to the [Unimarkup specification](https://github.com/unimarkup/specification/blob/main/configuration/merging-configurations.md).
    ///
    /// ## Arguments
    ///
    /// * `other` ... The configuration struct that is consumed and merged into **self**
    fn merge(&mut self, other: Self);

    /// Checks if all set values are valid.
    /// e.g. that set files exist
    fn validate(&self) -> Result<(), LogId>;

    /// Returns `true` if `validate()` returned `Ok`.
    fn is_valid(&self) -> bool {
        Self::validate(self).is_ok()
    }
}

#[derive(Parser, Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[command(name = UNIMARKUP_NAME, author, version, about, long_about = None)]
pub struct Config {
    #[command(flatten)]
    pub preamble: Preamble,
    #[command(flatten)]
    pub merging: MergingConfig,
    #[arg(index = 1)]
    pub input: PathBuf,
}

impl ConfigFns for Config {
    fn merge(&mut self, other: Self) {
        self.preamble.merge(other.preamble);
        self.merging.merge(other.merging);

        //Note: `input` is always taken from `self`
    }

    fn validate(&self) -> Result<(), LogId> {
        self.preamble.validate()?;
        self.merging.validate()?;

        if !self.input.exists() {
            return Err(set_event_with!(
                ConfigErrLogId::InvalidFile,
                &COMMONS_LOG_ID_MAP,
                &format!("Input file not found: {:?}", self.input)
            )
            .into());
        }
        Ok(())
    }
}

#[derive(Args, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MergingConfig {
    #[arg(long)]
    pub ignore_preamble: bool,
}

impl ConfigFns for MergingConfig {
    fn merge(&mut self, _other: Self) {
        // only main config counts for `bool` values according to specification
    }

    fn validate(&self) -> Result<(), LogId> {
        Ok(())
    }
}

pub fn parse_to_hashset<T>(s: &str) -> Result<HashSet<T>, clap::Error>
where
    T: std::str::FromStr + std::cmp::Eq + std::hash::Hash,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    if s.is_empty() {
        return Ok(HashSet::default());
    }

    let try_entries: Result<Vec<T>, _> = s.split(',').map(|e| T::from_str(e.trim())).collect();
    let entries = try_entries.map_err(|err| {
        clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            format!("HashSet conversion failed with: {:?}", err),
        )
    });
    Ok(HashSet::from_iter(entries?.into_iter()))
}

// Define extension trait
pub(crate) trait ReplaceIfNone<T> {
    fn replace_none(&mut self, other: Option<T>);
}

// Implement for Option<T>
impl<T> ReplaceIfNone<T> for Option<T> {
    fn replace_none(&mut self, other: Option<T>) {
        if self.is_none() {
            *self = other;
        }
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate__valid_config() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .canonicalize()
            .unwrap();
        path.push("tests/sample_files/empty.um");

        let cfg: Config = Config::parse_from(vec![
            "unimarkup",
            "--formats=html",
            "--title=\"Valid Config Test\"",
            path.to_str().unwrap(),
        ]);

        let result = cfg.validate();
        assert!(
            result.is_ok(),
            "Cause: {:?}",
            COMMONS_LOG_ID_MAP.get_entries(result.unwrap_err())
        );
    }

    #[should_panic]
    #[test]
    fn validate__invalid_config() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .canonicalize()
            .unwrap();
        path.push("tests/sample_files/empty.um");

        let cfg: Config = Config::parse_from(vec![
            "unimarkup",
            "--output-formats=html",
            //invalid attribute "shouldfail" on purpose
            "--style=shouldfail",
            path.to_str().unwrap(),
        ]);

        cfg.validate().unwrap();
    }

    #[should_panic]
    #[test]
    fn test__validate__invalid_multi_file_config() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .canonicalize()
            .unwrap();
        path.push("tests/sample_files/empty.um");

        let cfg: Config = Config::parse_from(vec![
            "unimarkup",
            "--output-formats=html",
            //invalid attribute "shouldfail" on purpose
            &format!("--fonts=shouldfail,{}", path.to_str().unwrap(),),
            path.to_str().unwrap(),
        ]);

        cfg.validate().unwrap();
    }
}
