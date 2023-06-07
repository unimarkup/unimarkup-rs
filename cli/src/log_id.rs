//! Defines log-ids for the cli crate

use logid::{ErrLogId, InfoLogId};
use thiserror::Error;

/// General error log-ids for the cli crate
#[derive(Debug, Clone, Error, ErrLogId)]
pub enum GeneralError {
    /// Log-id denoting a fail while reading a file
    #[error("Failed reading a file.")]
    FileRead,

    /// Log-id denoting a fail while writing to a file
    #[error("Failed writing to a file.")]
    FileWrite,

    /// Log-id denoting a fail while parsing CLI arguments
    #[error("Failed parsing given comandline arguments.")]
    ArgParse,

    /// Log-id denoting that compilation failed
    #[error("Failed compiling given input.")]
    Compile,
}

#[derive(Debug, Clone, InfoLogId)]
pub enum GeneralInfo {
    /// Log-id denoting that unimarkup-rs is writing to the output file
    WritingToFile,
    /// Log-id denoting that compilation finished
    FinishedCompiling,

    /// Log-id to notify log-thread to stop logging
    StopLogging,
}

impl std::fmt::Display for GeneralInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneralInfo::WritingToFile => write!(f, "Unimarkup is writing to a file."),
            GeneralInfo::FinishedCompiling => write!(f, "Unimarkup finished compiling."),
            GeneralInfo::StopLogging => write!(f, "Notify listeners to stop logging."),
        }
    }
}
