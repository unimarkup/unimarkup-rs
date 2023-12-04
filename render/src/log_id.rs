use logid::{evident::event::finalized::FinalizedEvent, log_id::LogId, ErrLogId, WarnLogId};
use thiserror::Error;

#[derive(Debug, Clone, ErrLogId, Error, PartialEq, Eq)]
pub enum RenderError {
    #[error("Rendering for this type is not implemented by the used renderer.")]
    Unimplemented,

    #[error("Output format `append()` failed. See log: '{}: {}'", .0.event_id, .0.entry_id)]
    BadAppend(FinalizedEvent<LogId>),
}

#[derive(Debug, Clone, WarnLogId)]
pub enum GeneralWarning {
    /// Log-id denoting a fail while reading a file
    FileRead,

    /// Log-id denoting the attempt to use an unsupported csl style
    UnsupportedCslStyle,

    /// Log-id denoting a failed json deserialization
    JSONDeserialization,

    /// Log-id denoting a failed json serialization
    JSONSerialization,
}

#[derive(Debug, Clone, ErrLogId, Error)]
pub enum CiteError {
    #[error("Importing the JavaScript module failed.")]
    ModuleImportError,

    #[error("Initialization of the CSL processor failed.")]
    ProcessorInitializationError,

    #[error("Processing the citations failed.")]
    CitationError,

    #[error("The check if footnotes exist, failed.")]
    CheckForFootnotesError,

    #[error("Getting the footnotes failed.")]
    GetFootnotesError,

    #[error("Getting the bibliography failed.")]
    GetBibliographyError,
}
