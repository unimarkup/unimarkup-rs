//! Defines log-ids for the cli crate

use logid::{
    id_map::LogIdMap,
    log_id::{self, EventLevel},
};
use once_cell::sync::Lazy;

/// Map to store [`LogId`]s set in the [`cli`] crate.
pub(crate) static CLI_LOG_ID_MAP: Lazy<LogIdMap> = Lazy::new(LogIdMap::new);

enum LogSubGrp {
    General = 0,
}

enum LogSubSubGrp {
    General = 0,
}

/// General error log-ids for the cli crate
#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum GeneralErrLogId {
    /// Log-id denoting a fail while reading a file
    FailedReadingFile = log_id::get_log_id(
        LogSubGrp::General as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Error,
        0,
    ),
    /// Log-id denoting a fail while writing to a file
    FailedWritingFile = log_id::get_log_id(
        LogSubGrp::General as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Error,
        1,
    ),
    /// Log-id denoting a fail while parsing a file
    FailedParsingArgs = log_id::get_log_id(
        LogSubGrp::General as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Error,
        2,
    ),
    /// Log-id denoting that compilation failed
    FailedCompiling = log_id::get_log_id(
        LogSubGrp::General as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Error,
        3,
    ),
}

#[derive(Debug)]
pub enum GeneralInfLogId {
    WritingToFile = log_id::get_log_id(
        LogSubGrp::General as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Info,
        0,
    ),
    FinishedCompiling = log_id::get_log_id(
        LogSubGrp::General as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Info,
        1,
    ),
}
