//! Defined log-ids for the frontend section

use logid::ErrLogId;

/// Parser error log-ids for the frontend section
#[derive(Debug, Clone, ErrLogId)]
pub enum ParserErrLogId {
    /// Log-id denoting that no Unimarkup element was detected
    NoUnimarkupDetected,
}

/// Parser warning log-ids for the frontend section
#[derive(Debug, Clone, ErrLogId)]
pub enum ParserWarnLogId {
    /// Log-id denoting an unsupported Unimarkup block
    UnsupportedBlock,
}
