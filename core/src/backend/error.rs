use crate::{log_id::LogId, error::CoreError, middleend::error::MiddleendError, elements::error::ElementError};

#[derive(Debug)]
pub enum BackendError {
    General(LogId),
    Loader(LogId),
    Renderer(LogId),
    Inline(LogId),
    Wrapped(LogId),
}

impl Into<LogId> for BackendError {
  fn into(self) -> LogId {
    match self {
      BackendError::General(log_id) => log_id,
      BackendError::Loader(log_id)=> log_id,
      BackendError::Renderer(log_id) => log_id,
      BackendError::Inline(log_id) => log_id,
      BackendError::Wrapped(log_id) => log_id,
    }
  }
}

impl Into<BackendError> for LogId {
  fn into(self) -> BackendError {
    BackendError::Wrapped(self)
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

impl From<MiddleendError> for BackendError {
    fn from(err: MiddleendError) -> Self {
      let log_id: LogId = err.into();
      log_id.into()
    }
}

impl From<ElementError> for BackendError {
  fn from(err: ElementError) -> Self {
    let log_id: LogId = err.into();
    log_id.into()
  }
}
