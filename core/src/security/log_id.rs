//! Defined log-ids for the security section

use crate::log_id::{get_log_id, LogKind, LogSubGrp, CORE_GRP};

enum LogSubSubGrp {
    Hashing = 1,
}

/// Hashing error log-ids for the security section
#[derive(Debug)]
pub enum HashingErrLogId {
    /// Log-id denoting that a file could not be read for hashing
    FailedReadingFile = get_log_id(
        CORE_GRP,
        LogSubGrp::Security as u8,
        LogSubSubGrp::Hashing as u8,
        LogKind::Error,
        0,
    ),
}
