use crate::log_id::LogId;


#[derive(Debug)]
pub enum CoreError {
  General(LogId),
  Frontend(LogId),
  Middleend(LogId),
  Backend(LogId),
  Elements(LogId),
}
