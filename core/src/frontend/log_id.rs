use crate::log_id::{get_log_id, CORE_GRP, LogSubGrp, LogKind};


enum LogSubSubGrp {
  General = 0,
  Parser = 1,
  Preamble = 2,
}

pub enum ParserErrLogId {
  NoUnimarkupDetected = get_log_id(CORE_GRP, LogSubGrp::Frontend as u8, LogSubSubGrp::Parser as u8, LogKind::Error, 0),
}

pub enum ParserWarnLogId {
  UnsupportedBlock = get_log_id(CORE_GRP, LogSubGrp::Frontend as u8, LogSubSubGrp::Parser as u8, LogKind::Warn, 1),
}

pub enum PreambleErrLogId {
  InvalidJSON = get_log_id(CORE_GRP, LogSubGrp::Frontend as u8, LogSubSubGrp::Preamble as u8, LogKind::Error, 0),
  InvalidYAML = get_log_id(CORE_GRP, LogSubGrp::Frontend as u8, LogSubSubGrp::Preamble as u8, LogKind::Error, 1),
}

