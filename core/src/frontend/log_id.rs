//! Defined log-ids for the frontend section

use crate::log_id::{get_log_id, CORE_GRP, LogSubGrp, LogKind};


enum LogSubSubGrp {
  Parser = 1,
  Preamble = 2,
}

/// Parser error log-ids for the frontend section
#[derive(Debug)]
pub enum ParserErrLogId {
  /// Log-id denoting that no Unimarkup element was detected
  NoUnimarkupDetected = get_log_id(CORE_GRP, LogSubGrp::Frontend as u8, LogSubSubGrp::Parser as u8, LogKind::Error, 0),
}

/// Parser warning log-ids for the frontend section
#[derive(Debug)]
pub enum ParserWarnLogId {
  /// Log-id denoting an unsupported Unimarkup block
  UnsupportedBlock = get_log_id(CORE_GRP, LogSubGrp::Frontend as u8, LogSubSubGrp::Parser as u8, LogKind::Warn, 1),
}

/// Preamble error log-ids for the frontend section
#[derive(Debug)]
pub enum PreambleErrLogId {
  /// Log-id denoting an invalid JSON
  InvalidJSON = get_log_id(CORE_GRP, LogSubGrp::Frontend as u8, LogSubSubGrp::Preamble as u8, LogKind::Error, 0),
  /// Log-id denoting an invalid YAML
  InvalidYAML = get_log_id(CORE_GRP, LogSubGrp::Frontend as u8, LogSubSubGrp::Preamble as u8, LogKind::Error, 1),
}

