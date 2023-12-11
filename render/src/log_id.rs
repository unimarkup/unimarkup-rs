use logid::{evident::event::finalized::FinalizedEvent, log_id::LogId, ErrLogId};
use thiserror::Error;

#[derive(Debug, Clone, ErrLogId, Error, PartialEq, Eq)]
pub enum RenderError {
    #[error("Rendering for this type is not implemented by the used renderer.")]
    Unimplemented,

    #[error("Output format `append()` failed. See log: '{}: {}'", .0.event_id, .0.entry_id)]
    BadAppend(FinalizedEvent<LogId>),

    #[error("Unexpected error during pdf render: {}", .0)]
    UnexpectedPdfError(String),
}
