//! Defines log-ids for the cli crate

use unimarkup_core::log_id::{get_log_id, LogKind};

/// Log-id main group number for the cli crate
pub const BIN_GRP: u8 = 1;


enum LogSubGrp {
  General = 0,
}

enum LogSubSubGrp {
  General = 0,
}

/// General error log-ids for the cli crate
#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum GeneralErrLogId {
  /// Log-id denoting a fail while reading a file
  FailedReadingFile = get_log_id(BIN_GRP, LogSubGrp::General as u8, LogSubSubGrp::General as u8, LogKind::Error, 0),
  /// Log-id denoting a fail while writing to a file
  FailedWritingFile = get_log_id(BIN_GRP, LogSubGrp::General as u8, LogSubSubGrp::General as u8, LogKind::Error, 1),
  /// Log-id denoting a fail while parsing a file
  FailedParsingArgs = get_log_id(BIN_GRP, LogSubGrp::General as u8, LogSubSubGrp::General as u8, LogKind::Error, 2),
  /// Log-id denoting that compilation failed
  FailedCompiling = get_log_id(BIN_GRP, LogSubGrp::General as u8, LogSubSubGrp::General as u8, LogKind::Error, 3),
}

#[derive(Debug)]
pub enum GeneralInfLogId {
  WritingToFile = get_log_id(BIN_GRP, LogSubGrp::General as u8, LogSubSubGrp::General as u8, LogKind::Info, 0),
  FinishedCompiling = get_log_id(BIN_GRP, LogSubGrp::General as u8, LogSubSubGrp::General as u8, LogKind::Info, 1),
}

