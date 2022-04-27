//! Defines errors for the core crate

use crate::log_id::LogId;

/// Error enum for the core crate
#[derive(Debug)]
pub enum CoreError {
    /// General error of the core crate
    General(LogId),
    /// Frontend error of the core crate
    Frontend(LogId),
    /// Middleened error of the core crate
    Middleend(LogId),
    /// Backend error of the core crate
    Backend(LogId),
    /// Element error of the core crate
    Element(LogId),
    /// Config error of the core crate
    Config(LogId),
    /// Security errir of the core crate
    Security(LogId),
}

impl From<CoreError> for LogId {
    fn from(err: CoreError) -> Self {
        match err {
            CoreError::General(log_id) => log_id,
            CoreError::Frontend(log_id) => log_id,
            CoreError::Middleend(log_id) => log_id,
            CoreError::Backend(log_id) => log_id,
            CoreError::Element(log_id) => log_id,
            CoreError::Config(log_id) => log_id,
            CoreError::Security(log_id) => log_id,
        }
    }
}

impl From<LogId> for CoreError {
    fn from(log_id: LogId) -> Self {
        CoreError::General(log_id)
    }
}

/// Error enum for the config section of the core crate
#[derive(Debug)]
pub enum ConfigError {
    /// General error of the config section
    General(LogId),
    /// Wrapped error of other sections that pass through the config section
    Wrapped(LogId),
}

impl From<ConfigError> for LogId {
    fn from(err: ConfigError) -> Self {
        match err {
            ConfigError::General(log_id) => log_id,
            ConfigError::Wrapped(log_id) => log_id,
        }
    }
}

impl From<LogId> for ConfigError {
    fn from(log_id: LogId) -> Self {
        ConfigError::General(log_id)
    }
}

impl From<ConfigError> for CoreError {
    fn from(err: ConfigError) -> Self {
        match err {
            ConfigError::General(log_id) => CoreError::Config(log_id),
            ConfigError::Wrapped(log_id) => CoreError::Config(log_id),
        }
    }
}
