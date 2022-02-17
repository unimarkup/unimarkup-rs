//! Defines errors for the frontend section

use crate::{
    elements::error::ElementError,
    error::{ConfigError, CoreError},
    log_id::LogId,
    middleend::error::MiddleendError,
};

/// Error enum for the frontend section
#[derive(Debug)]
pub enum FrontendError {
    /// General error of the frontend section
    General(LogId),
    /// Parser error of the frontend section
    Parser(LogId),
    /// Preamble error of the frontend section
    Preamble(LogId),
    /// Wrapped error of other sections that pass through the frontend section
    Wrapped(LogId),
}

impl From<FrontendError> for LogId {
    fn from(err: FrontendError) -> Self {
        match err {
            FrontendError::General(log_id) => log_id,
            FrontendError::Parser(log_id) => log_id,
            FrontendError::Preamble(log_id) => log_id,
            FrontendError::Wrapped(log_id) => log_id,
        }
    }
}

impl From<LogId> for FrontendError {
    fn from(log_id: LogId) -> Self {
        FrontendError::Wrapped(log_id)
    }
}

impl From<FrontendError> for CoreError {
    fn from(err: FrontendError) -> Self {
        match err {
            FrontendError::General(log_id) => CoreError::Frontend(log_id),
            FrontendError::Parser(log_id) => CoreError::Frontend(log_id),
            FrontendError::Preamble(log_id) => CoreError::Frontend(log_id),
            FrontendError::Wrapped(log_id) => CoreError::Frontend(log_id),
        }
    }
}

impl From<MiddleendError> for FrontendError {
    fn from(err: MiddleendError) -> Self {
        LogId::from(err).into()
    }
}

impl From<ElementError> for FrontendError {
    fn from(err: ElementError) -> Self {
        LogId::from(err).into()
    }
}

impl From<ConfigError> for FrontendError {
    fn from(err: ConfigError) -> Self {
        LogId::from(err).into()
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
