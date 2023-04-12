//! Defines log-ids for enclosed elements

use logid::log_id::{self, EventLevel};

use crate::{elements::log_id::LogIdSubGrp, log_id::LogIdMainGrp};

/// Enclosed error log-ids for the element section
#[derive(Debug)]
pub enum EnclosedErrLogId {
    /// Log-id denoting failed parsing
    FailedParsing = log_id::get_log_id(
        LogIdMainGrp::Element as u8,
        LogIdSubGrp::Enclosed as u8,
        EventLevel::Error,
        0,
    ),
}
