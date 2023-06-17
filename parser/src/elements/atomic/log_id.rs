//! Defines log-ids for atomic elements

use logid::ErrLogId;
use thiserror::Error;

/// Atomic error log-ids for the element section
#[derive(Debug, Clone, Error, ErrLogId)]
pub enum AtomicError {
    /// Log-id denoting an invalid heading level
    #[error("Invalid heading level detected.")]
    InvalidHeadingLvl,
}
