use crate::{log_id::LogId, error::CoreError};

#[derive(Debug)]
pub enum BackendError {
    General(LogId),
    Loader(LogId),
    Renderer(LogId),
    Inline(LogId),
}


impl From<BackendError> for CoreError {
    fn from(err: BackendError) -> Self {
      match err {
        BackendError::General(log_id) => CoreError::Backend(log_id),
        BackendError::Loader(log_id) => CoreError::Backend(log_id),
        BackendError::Renderer(log_id) => CoreError::Backend(log_id),
        BackendError::Inline(log_id) => CoreError::Backend(log_id),
      }
    }
}

