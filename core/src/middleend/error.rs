use crate::{log_id::LogId, error::CoreError};

#[derive(Debug)]
pub enum MiddleendError {
    General(LogId),
    Setup(LogId),
}


impl From<MiddleendError> for CoreError {
    fn from(err: MiddleendError) -> Self {
      match err {
        MiddleendError::General(log_id) => CoreError::Middleend(log_id),
        MiddleendError::Setup(log_id) => CoreError::Middleend(log_id),
      }
    }
}


