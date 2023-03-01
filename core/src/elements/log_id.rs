//! Defines log-ids for the element section

use logid::log_id::{self, EventLevel};

use crate::log_id::LogIdMainGrp;

pub(crate) enum LogIdSubGrp {
    General = 0,
    Atomic = 1,
    Enclosed = 2,
    Inline = 3,
    Preamble = 4,
}

/// General error log-ids for the element section
#[derive(Debug)]
pub enum GeneralErrLogId {
    /// Log-id denoting an invalid attribute
    InvalidAttribute = log_id::get_log_id(
        LogIdMainGrp::Element as u8,
        LogIdSubGrp::General as u8,
        EventLevel::Error,
        0,
    ),
    /// Log-id denoting an invalid element type
    InvalidElementType = log_id::get_log_id(
        LogIdMainGrp::Element as u8,
        LogIdSubGrp::General as u8,
        EventLevel::Error,
        1,
    ),
    /// Log-id denoting a failed block creation
    FailedBlockCreation = log_id::get_log_id(
        LogIdMainGrp::Element as u8,
        LogIdSubGrp::General as u8,
        EventLevel::Error,
        2,
    ),
    /// Log-id denoting failed inline parsing
    FailedInlineParsing = log_id::get_log_id(
        LogIdMainGrp::Element as u8,
        LogIdSubGrp::General as u8,
        EventLevel::Error,
        3,
    ),
}

/// Inline warning log-ids for the elements section
#[derive(Debug)]
pub enum InlineWarnLogId {
    /// Log-id denoting that inline parsing failed and content is treated as plain as fallback
    InlineParsingFailed = log_id::get_log_id(
        LogIdMainGrp::Element as u8,
        LogIdSubGrp::Inline as u8,
        EventLevel::Warn,
        1,
    ),
}

/// Inline error log-ids for the elements section
#[derive(Debug)]
pub enum InlineErrLogId {
    /// Log-id denoting that no inline elements were detected
    NoInlineDetected = log_id::get_log_id(
        LogIdMainGrp::Element as u8,
        LogIdSubGrp::Inline as u8,
        EventLevel::Error,
        1,
    ),
}

/// Preamble error log-ids for the frontend section
#[derive(Debug)]
pub enum PreambleErrLogId {
    /// Log-id denoting an invalid JSON
    InvalidJSON = log_id::get_log_id(
        LogIdMainGrp::Element as u8,
        LogIdSubGrp::Preamble as u8,
        EventLevel::Error,
        0,
    ),
    /// Log-id denoting an invalid YAML
    InvalidYAML = log_id::get_log_id(
        LogIdMainGrp::Element as u8,
        LogIdSubGrp::Preamble as u8,
        EventLevel::Error,
        1,
    ),
}
