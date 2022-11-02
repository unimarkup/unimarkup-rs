//! Defines log-ids for the middleend section

use logid::log_id::{self, EventLevel};

use crate::log_id::LogSubGrp;

enum LogSubSubGrp {
    General = 0,
    Setup = 1,
}

/// General error log-ids for the middleend section
#[derive(Debug)]
pub enum GeneralErrLogId {
    /// Log-id denoting a failed value insertion
    FailedValueInsert = log_id::get_log_id(
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Error,
        0,
    ),
    /// Log-id denoting a failed value update
    FailedValueUpdate = log_id::get_log_id(
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Error,
        1,
    ),
    /// Log-id denoting a failed row query
    FailedRowQuery = log_id::get_log_id(
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Error,
        2,
    ),
}

/// Setup error log-ids for the middleend section
#[derive(Debug)]
pub enum SetupErrLogId {
    /// Log-id denoting a failed database connection
    FailedDatabaseConnection = log_id::get_log_id(
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::Setup as u8,
        EventLevel::Error,
        0,
    ),
    /// Log-id denoting a failed table creation
    FailedTableCreation = log_id::get_log_id(
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::Setup as u8,
        EventLevel::Error,
        1,
    ),
}

/// General info log-ids for the middleend section
#[derive(Debug)]
pub enum GeneralInfLogId {
    /// Log-id denoting that an entry is overwritten
    EntryOverwritten = log_id::get_log_id(
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Info,
        0,
    ),
}

/// General warning log-ids for the middleend section
#[derive(Debug)]
pub enum GeneralWarnLogId {
    /// Log-id denoting that an entry is overwritten
    EntryOverwritten = log_id::get_log_id(
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Warn,
        0,
    ),
}

/// General debug log-ids for the middleend section
#[derive(Debug)]
pub enum GeneralDebugLogId {
    /// Log-id denoting that an entry already exists
    EntryAlreadyExists = log_id::get_log_id(
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::General as u8,
        EventLevel::Debug,
        0,
    ),
}
