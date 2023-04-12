//! Defines log-ids for atomic elements

use logid::log_id::{self, EventLevel};

use crate::{elements::log_id::LogIdSubGrp, log_id::LogIdMainGrp};

/// Atomic error log-ids for the element section
#[derive(Debug)]
pub enum AtomicErrLogId {
    /// Log-id denoting an invalid heading level
    InvalidHeadingLvl = log_id::get_log_id(
        LogIdMainGrp::Element as u8,
        LogIdSubGrp::Atomic as u8,
        EventLevel::Error,
        0,
    ),
}
