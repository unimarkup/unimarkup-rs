use logid::ErrLogId;
use thiserror::Error;

/// Log-ids for config errors
#[derive(Debug, Clone, Error, ErrLogId)]
pub enum ConfigErr {
    /// Log-id denoting an invalid file in the config
    #[error("Invalid file was given in config.")]
    InvalidFile,
    /// Log-id denoting an invalid config
    #[error("Config is invalid.")]
    InvalidConfig,
}
