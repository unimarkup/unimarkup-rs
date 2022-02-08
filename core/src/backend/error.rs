use crate::{log_id::LogId, error::CoreError};

#[derive(Debug)]
pub enum BackendError {
    General(LogId),
    Loader(LogId),
    Renderer(LogId),
    Inline(LogId),
    Wrapped(LogId),
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

impl From<CoreError> for BackendError {
  fn from(err: CoreError) -> Self {
    match err {
        CoreError::General(log_id) => Self::Wrapped(log_id),
        CoreError::Frontend(log_id) => Self::Wrapped(log_id),
        CoreError::Backend(log_id) => Self::Wrapped(log_id),
        CoreError::Middleend(log_id) => Self::Wrapped(log_id),
        CoreError::Elements(log_id) => Self::Wrapped(log_id),
    }
  }
}
