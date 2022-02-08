use crate::{log_id::LogId};


#[derive(Debug)]
pub enum CoreError {
  General(LogId),
  Frontend(LogId),
  Middleend(LogId),
  Backend(LogId),
  Element(LogId),
  Config(LogId),
}

impl Into<LogId> for CoreError {
  fn into(self) -> LogId {
    match self {
      CoreError::General(log_id) => log_id,
      CoreError::Frontend(log_id) => log_id,
      CoreError::Middleend(log_id) => log_id,
      CoreError::Backend(log_id) => log_id,
      CoreError::Element(log_id) => log_id,
      CoreError::Config(log_id) => log_id,
    }
  }
}

impl Into<CoreError> for LogId {
  fn into(self) -> CoreError {
    CoreError::General(self)
  }
}

#[derive(Debug)]
pub enum ConfigError {
  General(LogId),
  Wrapped(LogId),
}

impl Into<LogId> for ConfigError {
  fn into(self) -> LogId {
    match self {
      ConfigError::General(log_id) => log_id,
      ConfigError::Wrapped(log_id) => log_id,
    }
  }
}

impl Into<ConfigError> for LogId {
  fn into(self) -> ConfigError {
    ConfigError::General(self)
  }
}

impl From<ConfigError> for CoreError {
  fn from(err: ConfigError) -> Self {
    match err {
      ConfigError::General(log_id) => CoreError::Config(log_id),
      ConfigError::Wrapped(log_id) => CoreError::Config(log_id),
    }
  }
}
