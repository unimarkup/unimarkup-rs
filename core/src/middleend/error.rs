//! Defines errors for the middleend section

use crate::{error::CoreError, log_id::LogId};

/// Error enum for the middleend section
#[derive(Debug)]
pub enum MiddleendError {
    /// General error of the middleend section
    General(LogId),
    /// Setup error of the middleend section
    Setup(LogId),
    /// Wrapped error of other sections that pass through the middleend section
    Wrapped(LogId),
}

impl From<MiddleendError> for LogId {
    fn from(err: MiddleendError) -> Self {
        match err {
            MiddleendError::General(log_id) => log_id,
            MiddleendError::Setup(log_id) => log_id,
            MiddleendError::Wrapped(log_id) => log_id,
        }
    }
}

impl From<LogId> for MiddleendError {
    fn from(log_id: LogId) -> Self {
        MiddleendError::Wrapped(log_id)
    }
}

impl From<MiddleendError> for CoreError {
    fn from(err: MiddleendError) -> Self {
        match err {
            MiddleendError::General(log_id) => CoreError::Middleend(log_id),
            MiddleendError::Setup(log_id) => CoreError::Middleend(log_id),
            MiddleendError::Wrapped(log_id) => CoreError::Middleend(log_id),
        }
    }
}
