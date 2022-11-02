//! Defined log-ids for the frontend section

use logid::log_id::{self, EventLevel};

use crate::log_id::LogSubGrp;

enum LogSubSubGrp {
    Parser = 1,
}

/// Parser error log-ids for the frontend section
#[derive(Debug)]
pub enum ParserErrLogId {
    /// Log-id denoting that no Unimarkup element was detected
    NoUnimarkupDetected = log_id::get_log_id(
        LogSubGrp::Frontend as u8,
        LogSubSubGrp::Parser as u8,
        EventLevel::Error,
        0,
    ),
}

/// Parser warning log-ids for the frontend section
#[derive(Debug)]
pub enum ParserWarnLogId {
    /// Log-id denoting an unsupported Unimarkup block
    UnsupportedBlock = log_id::get_log_id(
        LogSubGrp::Frontend as u8,
        LogSubSubGrp::Parser as u8,
        EventLevel::Warn,
        1,
    ),
}
