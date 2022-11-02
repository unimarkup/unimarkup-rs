//! Defines log-ids for the cli crate

use logid::{
    id_map::LogIdMap,
    log_id::{self, EventLevel},
};
use once_cell::sync::Lazy;

/// Map to store [`LogId`]s set in the [`cli`] crate.
pub(crate) static CLI_LOG_ID_MAP: Lazy<LogIdMap> = Lazy::new(LogIdMap::new);

enum LogIdMainGrp {
    General = 0,
}

enum LogIdSubGrp {
    General = 0,
}

/// General error log-ids for the cli crate
#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum GeneralErrLogId {
    /// Log-id denoting a fail while reading a file
    FailedReadingFile = log_id::get_log_id(
        LogIdMainGrp::General as u8,
        LogIdSubGrp::General as u8,
        EventLevel::Error,
        0,
    ),
    /// Log-id denoting a fail while writing to a file
    FailedWritingFile = log_id::get_log_id(
        LogIdMainGrp::General as u8,
        LogIdSubGrp::General as u8,
        EventLevel::Error,
        1,
    ),
    /// Log-id denoting a fail while parsing a file
    FailedParsingArgs = log_id::get_log_id(
        LogIdMainGrp::General as u8,
        LogIdSubGrp::General as u8,
        EventLevel::Error,
        2,
    ),
    /// Log-id denoting that compilation failed
    FailedCompiling = log_id::get_log_id(
        LogIdMainGrp::General as u8,
        LogIdSubGrp::General as u8,
        EventLevel::Error,
        3,
    ),
}

#[derive(Debug)]
pub enum GeneralInfLogId {
    /// Log-id denoting that unimarkup-rs is writing to the output file
    WritingToFile = log_id::get_log_id(
        LogIdMainGrp::General as u8,
        LogIdSubGrp::General as u8,
        EventLevel::Info,
        0,
    ),
    /// Log-id denoting that compilation finished
    FinishedCompiling = log_id::get_log_id(
        LogIdMainGrp::General as u8,
        LogIdSubGrp::General as u8,
        EventLevel::Info,
        1,
    ),
}
