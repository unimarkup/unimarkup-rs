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
}

#[allow(clippy::from_over_into)]
impl Into<LogId> for CoreError {
    fn into(self) -> LogId {
        match self {
            CoreError::General(log_id) => log_id,
            CoreError::Frontend(log_id) => log_id,
            CoreError::Middleend(log_id) => log_id,
            CoreError::Backend(log_id) => log_id,
            CoreError::Element(log_id) => log_id,
            CoreError::Config(log_id) => log_id,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<CoreError> for LogId {
    fn into(self) -> CoreError {
        CoreError::General(self)
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

#[allow(clippy::from_over_into)]
impl Into<LogId> for ConfigError {
    fn into(self) -> LogId {
        match self {
            ConfigError::General(log_id) => log_id,
            ConfigError::Wrapped(log_id) => log_id,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<ConfigError> for LogId {
    fn into(self) -> ConfigError {
        ConfigError::General(self)
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
