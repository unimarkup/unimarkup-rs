use std::{collections::HashSet, path::PathBuf};

use clap::{Args, Parser};
use serde::{Deserialize, Serialize};

use self::preamble::Preamble;

pub mod output;
pub mod preamble;

const UNIMARKUP_NAME: &str = "unimarkup";

#[derive(Parser, Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[command(name = UNIMARKUP_NAME, author, version, about, long_about = None)]
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
