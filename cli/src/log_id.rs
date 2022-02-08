use unimarkup_core::log_id::{get_log_id, LogKind};


pub const BIN_GRP: u8 = 1;


enum LogSubGrp {
  General = 0,
}

enum LogSubSubGrp {
  General = 0,
}


#[derive(Debug)]
pub enum GeneralErrLogId {
  FailedReadingFile = get_log_id(BIN_GRP, LogSubGrp::General as u8, LogSubSubGrp::General as u8, LogKind::Error, 0),
  FailedWritingFile = get_log_id(BIN_GRP, LogSubGrp::General as u8, LogSubSubGrp::General as u8, LogKind::Error, 1),
  FailedParsingArgs = get_log_id(BIN_GRP, LogSubGrp::General as u8, LogSubSubGrp::General as u8, LogKind::Error, 2),
  FailedCompiling = get_log_id(BIN_GRP, LogSubGrp::General as u8, LogSubSubGrp::General as u8, LogKind::Error, 3),
}

#[derive(Debug)]
pub enum GeneralInfLogId {
  WritingToFile = get_log_id(BIN_GRP, LogSubGrp::General as u8, LogSubSubGrp::General as u8, LogKind::Info, 0),
  FinishedCompiling = get_log_id(BIN_GRP, LogSubGrp::General as u8, LogSubSubGrp::General as u8, LogKind::Info, 1),
}

