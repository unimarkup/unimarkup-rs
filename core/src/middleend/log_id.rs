//! Defines log-ids for the middleend section

use crate::log_id::{get_log_id, LogKind, LogSubGrp, CORE_GRP};

enum LogSubSubGrp {
    General = 0,
    Setup = 1,
}

/// General error log-ids for the middleend section
#[derive(Debug)]
pub enum GeneralErrLogId {
    /// Log-id denoting a failed value insertion
    FailedValueInsert = get_log_id(
        CORE_GRP,
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::General as u8,
        LogKind::Error,
        0,
    ),
    /// Log-id denoting a failed value update
    FailedValueUpdate = get_log_id(
        CORE_GRP,
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::General as u8,
        LogKind::Error,
        1,
    ),
    /// Log-id denoting a failed row query
    FailedRowQuery = get_log_id(
        CORE_GRP,
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::General as u8,
        LogKind::Error,
        2,
    ),
}

/// Setup error log-ids for the middleend section
#[derive(Debug)]
pub enum SetupErrLogId {
    /// Log-id denoting a failed database connection
    FailedDatabaseConnection = get_log_id(
        CORE_GRP,
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::Setup as u8,
        LogKind::Error,
        0,
    ),
    /// Log-id denoting a failed table creation
    FailedTableCreation = get_log_id(
        CORE_GRP,
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::Setup as u8,
        LogKind::Error,
        1,
    ),
}

/// General info log-ids for the middleend section
#[derive(Debug)]
pub enum GeneralInfLogId {
    /// Log-id denoting that an entry is overwritten
    EntryOverwritten = get_log_id(
        CORE_GRP,
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::General as u8,
        LogKind::Info,
        0,
    ),
}

/// General warning log-ids for the middleend section
#[derive(Debug)]
pub enum GeneralWarnLogId {
    /// Log-id denoting that an entry is overwritten
    EntryOverwritten = get_log_id(
        CORE_GRP,
        LogSubGrp::Middleend as u8,
        LogSubSubGrp::General as u8,
        LogKind::Warn,
        0,
    ),
}
