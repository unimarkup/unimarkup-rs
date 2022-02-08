use unimarkup_core::{log_id::LogId, error::CoreError};


#[derive(Debug)]
pub enum CliError {
  General(LogId),
  Core(LogId),
  Wrapped(LogId),
}

impl Into<LogId> for CliError {
  fn into(self) -> LogId {
    match self {
      CliError::General(log_id) => log_id,
      CliError::Core(log_id) => log_id,
      CliError::Wrapped(log_id) => log_id,
    }
  }
}

impl Into<CliError> for LogId {
  fn into(self) -> CliError {
    CliError::Wrapped(self)
  }
}

impl From<CoreError> for CliError {
  fn from(err: CoreError) -> Self {
    match err {
        CoreError::General(log_id) => CliError::Core(log_id),
        CoreError::Frontend(log_id) => CliError::Core(log_id),
        CoreError::Middleend(log_id) => CliError::Core(log_id),
        CoreError::Backend(log_id) => CliError::Core(log_id),
        CoreError::Element(log_id) => CliError::Core(log_id),
        CoreError::Config(log_id) => CliError::Core(log_id),
    }
  }
}

// impl fmt::Display for UmError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             UmError::Syntax(err) => err.fmt(f),
//             UmError::Ir(err) => err.fmt(f),
//             UmError::Backend(err) => err.fmt(f),
//             UmError::General { msg, error } => f.write_fmt(format_args!("{}:\n {}", msg, error)),
//         }
//     }
// }
