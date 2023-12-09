use std::{collections::HashSet, path::PathBuf};
use std::error::Error;
use std::str::FromStr;

use clap::{Args, Parser};
use logid::{err, log};
use serde::{Deserialize, Serialize};

use self::{log_id::ConfigErr, output::Output, preamble::Preamble};

pub use icu_locid;
use icu_locid::{Locale, locale};

pub mod locale;
pub mod log_id;
pub mod output;
pub mod preamble;

const UNIMARKUP_NAME: &str = "unimarkup";
const ABOUT: &str = "The official compiler for Unimarkup.";
const HELP_TEMPLATE: &str = r#"
{before-help}{name} {version} - {about-with-newline}
Written by: {author-with-newline}
{usage-heading} {usage}

{all-args}{after-help}"#;

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
    fn validate(&self) -> Result<(), ConfigErr>;

    /// Returns `true` if `validate()` returned `Ok`.
    fn is_valid(&self) -> bool {
        ConfigFns::validate(self).is_ok()
    }
}

#[derive(Parser, Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[command(name = UNIMARKUP_NAME, help_template = HELP_TEMPLATE, author, version, about = ABOUT, long_about = None)]
pub struct Config {
    #[command(flatten)]
    pub preamble: Preamble,
    #[command(flatten)]
    pub output: Output,
    #[command(flatten)]
    pub merging: MergingConfig,
    #[arg(index = 1)]
    pub input: PathBuf,
}

impl ConfigFns for Config {
    fn merge(&mut self, other: Self) {
        self.preamble.merge(other.preamble);
        self.output.merge(other.output);
        self.merging.merge(other.merging);

        //Note: `input` is always taken from `self`
    }

    fn validate(&self) -> Result<(), ConfigErr> {
        self.preamble.validate()?;
        self.output.validate()?;
        self.merging.validate()?;

        if !self.input.exists() {
            return err!(
                ConfigErr::InvalidFile,
                format!("Input file not found: {:?}", self.input)
            );
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

    fn validate(&self) -> Result<(), ConfigErr> {
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
    Ok(HashSet::from_iter(entries?))
}

fn parse_locale_path_buf(s: &str) -> Result<(Locale, PathBuf), Box<dyn Error + Send + Sync + 'static>>
{
    if s.is_empty() {
        return Ok((locale!("en"), PathBuf::from_str("invalid path").unwrap()));
    }
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    let mut locale = locale!("en");
    match s[..pos].parse::<Locale>() {
        Ok(l) => locale = l,
        Err(e) => {
            log!(
                ConfigErr::InvalidFile,
                format!("Parsing the locale failed with error: '{:?}'", e)
            );
        }
    };
    let path_buf: PathBuf = s[pos + 1..]
        .parse()
        .unwrap();
    Ok((locale, path_buf))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_config() {
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
        assert!(result.is_ok(), "Valid config was not recognized as valid.");
    }

    #[should_panic]
    #[test]
    fn invalid_config() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .canonicalize()
            .unwrap();
        path.push("tests/sample_files/empty.um");

        let cfg: Config = Config::parse_from(vec![
            "unimarkup",
            "--output-formats=html",
            //invalid attribute "shouldfail" on purpose
            "--citation-style=shouldfail",
            path.to_str().unwrap(),
        ]);

        cfg.validate().unwrap();
    }

    #[should_panic]
    #[test]
    fn invalid_multi_file_config() {
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
