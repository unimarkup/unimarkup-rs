//! Log-id handling functionality for unimarkup-rs

use logid::{
    id_map::LogIdMap,
    log_id::{self, EventLevel},
};
use once_cell::sync::Lazy;

/// Map to store [`LogId`]s set in the [`core`] crate.
pub(crate) static CORE_LOG_ID_MAP: Lazy<LogIdMap> = Lazy::new(LogIdMap::new);

/// Log-id sub group for the core crate
pub enum LogSubGrp {
    /// General log-id sub group
    General = 0,
    /// Frontend log-id sub group
    Frontend = 1,
    /// Middleend log-id sub group
    Middleend = 2,
    /// Backend log-id sub group
    Backend = 3,
    /// Element log-id sub group
    Element = 4,
    /// Config log-id sub group
    Config = 5,
    /// Security log-id sub group
    Security = 6,
}

enum SubSubGrp {
    General = 0,
}

/// Log-ids for config errors
pub enum ConfigErrLogId {
    /// Log-id denoting an invalid file in the config
    InvalidFile = log_id::get_log_id(
        LogSubGrp::Config as u8,
        SubSubGrp::General as u8,
        EventLevel::Error,
        0,
    ),
    /// Log-id denoting an invalid path in the config
    InvalidPath = log_id::get_log_id(
        LogSubGrp::Config as u8,
        SubSubGrp::General as u8,
        EventLevel::Error,
        1,
    ),
    /// Log-id denoting an invalid config
    InvalidConfig = log_id::get_log_id(
        LogSubGrp::Config as u8,
        SubSubGrp::General as u8,
        EventLevel::Error,
        2,
    ),
}
