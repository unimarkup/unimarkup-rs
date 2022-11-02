//! Defined log-ids for the security section

use logid::log_id::{self, EventLevel};

use crate::log_id::LogSubGrp;

enum LogSubSubGrp {
    Hashing = 1,
}

/// Hashing error log-ids for the security section
#[derive(Debug)]
pub enum HashingErrLogId {
    /// Log-id denoting that a file could not be read for hashing
    FailedReadingFile = log_id::get_log_id(
        LogSubGrp::Security as u8,
        LogSubSubGrp::Hashing as u8,
        EventLevel::Error,
        0,
    ),
}
