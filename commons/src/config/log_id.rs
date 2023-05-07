use logid::log_id::EventLevel;

use crate::log_id::LogIdMainGrp;

enum LogIdSubGrp {
    General = 0,
}

/// Log-ids for config errors
pub enum ConfigErrLogId {
    /// Log-id denoting an invalid file in the config
    InvalidFile = logid::log_id::get_log_id(
        LogIdMainGrp::Config as u8,
        LogIdSubGrp::General as u8,
        EventLevel::Error,
        0,
    ),
    /// Log-id denoting an invalid config
    InvalidConfig = logid::log_id::get_log_id(
        LogIdMainGrp::Config as u8,
        LogIdSubGrp::General as u8,
        EventLevel::Error,
        2,
    ),
}
