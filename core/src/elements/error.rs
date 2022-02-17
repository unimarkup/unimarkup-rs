//! Defines errors for the element section

use crate::{error::CoreError, log_id::LogId};

/// Error enum for the element section
#[derive(Debug)]
pub enum ElementError {
    /// General error of the element section
    General(LogId),
    /// Atomic error of the element section
    Atomic(LogId),
    /// Enclosed error of the element section
    Enclosed(LogId),
    /// MetaData error of the element section
    MetaData(LogId),
    /// Wrapped error of the other sections that pass through the element section
    Wrapped(LogId),
}

impl From<ElementError> for LogId {
    fn from(err: ElementError) -> Self {
        match err {
            ElementError::General(log_id) => log_id,
            ElementError::Atomic(log_id) => log_id,
            ElementError::Enclosed(log_id) => log_id,
            ElementError::MetaData(log_id) => log_id,
            ElementError::Wrapped(log_id) => log_id,
        }
    }
}

impl From<LogId> for ElementError {
    fn from(log_id: LogId) -> Self {
        ElementError::Wrapped(log_id)
    }
}

impl From<ElementError> for CoreError {
    fn from(err: ElementError) -> Self {
        match err {
            ElementError::General(log_id) => CoreError::Element(log_id),
            ElementError::Atomic(log_id) => CoreError::Element(log_id),
            ElementError::Enclosed(log_id) => CoreError::Element(log_id),
            ElementError::MetaData(log_id) => CoreError::Element(log_id),
            ElementError::Wrapped(log_id) => CoreError::Element(log_id),
        }
    }
}

/// Error enum for the MetaData section
#[derive(Debug)]
pub enum MetaDataError {
    /// General error of the MetaData section
    General(LogId),
    /// Wrapped error of other sections that pass through the element section
    Wrapped(LogId),
}

impl From<MetaDataError> for LogId {
    fn from(err: MetaDataError) -> Self {
        match err {
            MetaDataError::General(log_id) => log_id,
            MetaDataError::Wrapped(log_id) => log_id,
        }
    }
}

impl From<LogId> for MetaDataError {
    fn from(log_id: LogId) -> Self {
        MetaDataError::Wrapped(log_id)
    }
}
