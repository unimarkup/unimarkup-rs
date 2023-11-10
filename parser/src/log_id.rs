//! Defined parser log-ids

use logid::{ErrLogId, WarnLogId};

/// Parser error log-ids
#[derive(Debug, Clone, ErrLogId)]
pub enum ParserError {
    /// Log-id denoting that no Unimarkup element was detected
    NoUnimarkupDetected,
}

/// Parser warning log-ids
#[derive(Debug, Clone, WarnLogId)]
pub enum ParserWarning {
    /// Log-id denoting an unsupported Unimarkup block
    UnsupportedBlock,
}
