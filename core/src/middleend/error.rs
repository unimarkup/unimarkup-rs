use crate::{log_id::LogId, error::CoreError};

#[derive(Debug)]
pub enum MiddleendError {
    General(LogId),
    Setup(LogId),
    Wrapped(LogId),
}

impl Into<LogId> for MiddleendError {
  fn into(self) -> LogId {
    match self {
      MiddleendError::General(log_id) => log_id,
      MiddleendError::Setup(log_id) => log_id,
      MiddleendError::Wrapped(log_id) => log_id,
    }
  }
}

impl Into<MiddleendError> for LogId {
  fn into(self) -> MiddleendError {
    MiddleendError::Wrapped(self)
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
