//! Defines log-ids for the backend section

use crate::log_id::{get_log_id, LogKind, LogSubGrp, CORE_GRP};

enum LogSubSubGrp {
    Loader = 1,
    Inline = 3,
}

/// Loader error log-ids for the backend section
#[derive(Debug)]
pub enum LoaderErrLogId {
    /// Log-id denoting an invalid element type
    InvalidElementType = get_log_id(
        CORE_GRP,
        LogSubGrp::Backend as u8,
        LogSubSubGrp::Loader as u8,
        LogKind::Error,
        0,
    ),
}

/// Inline error log-ids for the backend section
#[derive(Debug)]
pub enum InlineErrLogId {
    /// Log-id denoting that no inline elements were detected
    NoInlineDetected = get_log_id(
        CORE_GRP,
        LogSubGrp::Backend as u8,
        LogSubSubGrp::Inline as u8,
        LogKind::Error,
        0,
    ),
}
