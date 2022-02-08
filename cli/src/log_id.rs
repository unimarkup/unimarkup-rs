use unimarkup_core::log_id::{get_log_id, CORE_GRP, LogKind};


enum LogSubGrp {
  General = 0,
}

enum LogSubSubGrp {
  FileHandling = 0,
}


#[derive(Debug)]
pub enum GeneralErrLogId {
  FailedReadingFile = get_log_id(CORE_GRP, LogSubGrp::General as u8, LogSubSubGrp::FileHandling as u8, LogKind::Error, 0),
  FailedWritingFile = get_log_id(CORE_GRP, LogSubGrp::General as u8, LogSubSubGrp::FileHandling as u8, LogKind::Error, 1),
}

