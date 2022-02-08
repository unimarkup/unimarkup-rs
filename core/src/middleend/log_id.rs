use crate::log_id::{get_log_id, CORE_GRP, LogSubGrp, LogKind};


enum LogSubSubGrp {
  General = 0,
  Setup = 1,
}

#[derive(Debug)]
pub enum GeneralErrLogId {
  FailedValueInsert = get_log_id(CORE_GRP, LogSubGrp::Middleend as u8, LogSubSubGrp::General as u8, LogKind::Error, 0),
  FailedValueUpdate = get_log_id(CORE_GRP, LogSubGrp::Middleend as u8, LogSubSubGrp::General as u8, LogKind::Error, 1),
  FailedRowQuery = get_log_id(CORE_GRP, LogSubGrp::Middleend as u8, LogSubSubGrp::General as u8, LogKind::Error, 2),
}

#[derive(Debug)]
pub enum SetupErrLogId {
  FailedDatabaseConnection = get_log_id(CORE_GRP, LogSubGrp::Middleend as u8, LogSubSubGrp::Setup as u8, LogKind::Error, 0),
  FailedTableCreation = get_log_id(CORE_GRP, LogSubGrp::Middleend as u8, LogSubSubGrp::Setup as u8, LogKind::Error, 1),
}
