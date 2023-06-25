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
    /// Log-id denoting failure when creating a file
    #[error("File could not be created")]
    FileCreate,
    /// Log-id denoting failure to download locales file
    #[error("Locales file could not be downloaded.")]
    LocaleDownload,
    /// Log-id denoting provided locales are not included in default locales data
    #[error("Given locale is not in default locales data. Please provide data or use one of supported default locales: en-US, de-AT, bs-BA")]
    BadLocaleUsed,
    /// Log-id denoting provided locales are not included in default locales data
    #[error("Given locale is incompatible with data file. Using falling locale '{}'.", .0)]
    LocaleMissingKeys(String),
}
