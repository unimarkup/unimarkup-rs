//! Defines errors for the security section

use crate::{error::CoreError, log_id::LogId};

/// Error enum for the security section
#[derive(Debug)]
pub enum SecurityError {
    /// Hashing error of the security section
    Hashing(LogId),
}

impl From<SecurityError> for LogId {
    fn from(err: SecurityError) -> Self {
        match err {
            SecurityError::Hashing(log_id) => log_id,
        }
    }
}

impl From<LogId> for SecurityError {
    fn from(log_id: LogId) -> Self {
        SecurityError::Hashing(log_id)
    }
}

impl From<SecurityError> for CoreError {
    fn from(err: SecurityError) -> Self {
        match err {
            SecurityError::Hashing(log_id) => CoreError::Security(log_id),
        }
    }
}
