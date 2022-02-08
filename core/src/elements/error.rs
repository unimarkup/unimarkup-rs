use crate::{log_id::LogId, error::CoreError};

#[derive(Debug)]
pub enum ElementError {
    General(LogId),
    Atomic(LogId),
    Enclosed(LogId),
    MetaData(LogId),
    Wrapped(LogId),
}

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

#[derive(Debug)]
pub enum MetaDataError {
  General(LogId),
  Wrapped(LogId),
}

impl Into<LogId> for MetaDataError {
  fn into(self) -> LogId {
    match self {
      MetaDataError::General(log_id) => log_id,
      MetaDataError::Wrapped(log_id) => log_id,
    }
  }
}

impl Into<MetaDataError> for LogId {
  fn into(self) -> MetaDataError {
    MetaDataError::Wrapped(self)
  }
}

