use crate::{log_id::{LogId, NO_LOG_ID}, error::CoreError};

#[derive(Debug)]
pub enum FrontendError {
    General(LogId),
    Parser(LogId),
    Preamble(LogId),
}


impl From<FrontendError> for CoreError {
    fn from(err: FrontendError) -> Self {
      match err {
        FrontendError::General(log_id) => CoreError::Frontend(log_id),
        FrontendError::Parser(log_id) => CoreError::Frontend(log_id),
        FrontendError::Preamble(log_id) => CoreError::Frontend(log_id),
      }
    }
}

impl From<CoreError> for FrontendError {
  fn from(err: CoreError) -> Self {
    match err {
        CoreError::General(log_id) => Self::General(log_id),
        CoreError::Frontend(log_id) => Self::General(log_id),
        _ => Self::General(NO_LOG_ID),
    }
  }
}

/// Uses a custom [`pest::error::Error`] to display parsing errors using Pest's pretty print.
///
/// # Arguments
///
/// * `msg` - Custom error message
/// * `span` - Span in input Unimarkup document where this specific error occured
pub fn custom_pest_error(msg: impl Into<String>, span: pest::Span) -> String {
  use crate::frontend::parser;
  use pest::error;

  let error = error::Error::new_from_span(
      error::ErrorVariant::<parser::Rule>::CustomError {
          message: msg.into(),
      },
      span,
  );

  error.to_string()
}
