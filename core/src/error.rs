use crate::log_id::LogId;


#[derive(Debug)]
pub enum CoreError {
  General(LogId),
  Frontend(LogId),
  Middleend(LogId),
  Backend(LogId),
  Elements(LogId),
}




#[derive(Debug)]
pub enum FrontendError {
  General(LogId),
}

#[derive(Debug)]
pub enum MiddleendError {
  General(LogId),
}

#[derive(Debug)]
pub enum BackendError {
  General(LogId),
}

impl From<FrontendError> for Error {
    fn from(err: FrontendError) -> Self {
      match err {
        FrontendError::General(log_id) => Error::Frontend(log_id)
      }
    }
}
