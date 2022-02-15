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

#[allow(clippy::from_over_into)]
impl Into<LogId> for ElementError {
    fn into(self) -> LogId {
        match self {
            ElementError::General(log_id) => log_id,
            ElementError::Atomic(log_id) => log_id,
            ElementError::Enclosed(log_id) => log_id,
            ElementError::MetaData(log_id) => log_id,
            ElementError::Wrapped(log_id) => log_id,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<ElementError> for LogId {
    fn into(self) -> ElementError {
        ElementError::Wrapped(self)
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

#[allow(clippy::from_over_into)]
impl Into<LogId> for MetaDataError {
    fn into(self) -> LogId {
        match self {
            MetaDataError::General(log_id) => log_id,
            MetaDataError::Wrapped(log_id) => log_id,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<MetaDataError> for LogId {
    fn into(self) -> MetaDataError {
        MetaDataError::Wrapped(self)
    }
}
