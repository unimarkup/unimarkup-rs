//! Defines log-ids for the element section

use logid::log_id::{self, EventLevel};

use crate::log_id::LogSubGrp;

enum LogSubSubGrp {
    General = 0,
    Atomic = 1,
    Enclosed = 2,
    Inline = 3,
}

/// General error log-ids for the element section
#[derive(Debug)]
pub enum GeneralErrLogId {
    /// Log-id denoting an invalid attribute
    InvalidAttribute = log_id::get_log_id(
        LogSubGrp::Element as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Error,
        0,
    ),
    /// Log-id denoting an invalid element type
    InvalidElementType = log_id::get_log_id(
        LogSubGrp::Element as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Error,
        1,
    ),
    /// Log-id denoting a failed block creation
    FailedBlockCreation = log_id::get_log_id(
        LogSubGrp::Element as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Error,
        2,
    ),
    /// Log-id denoting failed inline parsing
    FailedInlineParsing = log_id::get_log_id(
        LogSubGrp::Element as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Error,
        3,
    ),
}

/// Atomic error log-ids for the element section
#[derive(Debug)]
pub enum AtomicErrLogId {
    /// Log-id denoting an invalid heading level
    InvalidHeadingLvl = log_id::get_log_id(
        LogSubGrp::Element as u8,
        LogSubSubGrp::Atomic as u8,
        EventLevel::Error,
        0,
    ),
}

/// Enclosed error log-ids for the element section
#[derive(Debug)]
pub enum EnclosedErrLogId {
    /// Log-id denoting failed parsing
    FailedParsing = log_id::get_log_id(
        LogSubGrp::Element as u8,
        LogSubSubGrp::Enclosed as u8,
        EventLevel::Error,
        0,
    ),
}

/// Inline warning log-ids for the elements section
#[derive(Debug)]
pub enum InlineWarnLogId {
    /// Log-id denoting that inline parsing failed and content is treated as plain as fallback
    InlineParsingFailed = log_id::get_log_id(
        LogSubGrp::Element as u8,
        LogSubSubGrp::Inline as u8,
        EventLevel::Warn,
        1,
    ),
}
