//! Defined log-ids for the security section

use logid::ErrLogId;
use thiserror::Error;

/// Hashing error log-ids for the security section
#[derive(Debug, Clone, Error, ErrLogId)]
pub enum HashingError {
    /// Log-id denoting that a file could not be read for hashing
    #[error("File could not be read.")]
    FailedReadingFile,
}
