//! Defines errors for the backend section

use crate::{elements::error::ElementError, error::CoreError, log_id::LogId};

/// Error enum for the backend section
#[derive(Debug)]
pub enum BackendError {
    /// General error of the backend section
    General(LogId),
    /// Loader error of the backend section
    Loader(LogId),
    /// Renderer error of the backend section
    Renderer(LogId),
    /// Inline error of the backend section
    Inline(LogId),
    /// Wrapped error of other sections that pass through the backend section
    Wrapped(LogId),
}

impl From<BackendError> for LogId {
    fn from(err: BackendError) -> Self {
        match err {
            BackendError::General(log_id) => log_id,
            BackendError::Loader(log_id) => log_id,
            BackendError::Renderer(log_id) => log_id,
            BackendError::Inline(log_id) => log_id,
            BackendError::Wrapped(log_id) => log_id,
        }
    }
}

impl From<LogId> for BackendError {
    fn from(log_id: LogId) -> Self {
        BackendError::Wrapped(log_id)
    }
}

impl From<BackendError> for CoreError {
    fn from(err: BackendError) -> Self {
        match err {
            BackendError::General(log_id) => CoreError::Backend(log_id),
            BackendError::Loader(log_id) => CoreError::Backend(log_id),
            BackendError::Renderer(log_id) => CoreError::Backend(log_id),
            BackendError::Inline(log_id) => CoreError::Backend(log_id),
            BackendError::Wrapped(log_id) => CoreError::Backend(log_id),
        }
    }
}

impl From<ElementError> for BackendError {
    fn from(err: ElementError) -> Self {
        let log_id: LogId = err.into();
        log_id.into()
    }
}
