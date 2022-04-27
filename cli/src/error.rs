//! Defines errors for the cli crate

use unimarkup_core::{error::CoreError, log_id::LogId};

/// Error enum for the cli crate
#[derive(Debug)]
pub enum CliError {
    /// General error of the cli crate
    General(LogId),
    /// Core error of the cli crate
    Core(LogId),
}

impl From<CliError> for LogId {
    fn from(err: CliError) -> Self {
        match err {
            CliError::General(log_id) => log_id,
            CliError::Core(log_id) => log_id,
        }
    }
}

impl From<CoreError> for CliError {
    fn from(err: CoreError) -> Self {
        match err {
            CoreError::General(log_id) => CliError::Core(log_id),
            CoreError::Frontend(log_id) => CliError::Core(log_id),
            CoreError::Middleend(log_id) => CliError::Core(log_id),
            CoreError::Backend(log_id) => CliError::Core(log_id),
            CoreError::Element(log_id) => CliError::Core(log_id),
            CoreError::Config(log_id) => CliError::Core(log_id),
            CoreError::Security(log_id) => CliError::Core(log_id),
        }
    }
}
