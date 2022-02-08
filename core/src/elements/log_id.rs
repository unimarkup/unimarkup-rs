use crate::log_id::{get_log_id, CORE_GRP, LogSubGrp, LogKind};


enum LogSubSubGrp {
  General = 0,
  Atomic = 1,
  Enclosed = 2,
  MetaData = 3,
}

#[derive(Debug)]
pub enum GeneralErrLogId {
  InvalidAttribute = get_log_id(CORE_GRP, LogSubGrp::Element as u8, LogSubSubGrp::General as u8, LogKind::Error, 0),
  InvalidElementType = get_log_id(CORE_GRP, LogSubGrp::Element as u8, LogSubSubGrp::General as u8, LogKind::Error, 1),
  FailedBlockCreation = get_log_id(CORE_GRP, LogSubGrp::Element as u8, LogSubSubGrp::General as u8, LogKind::Error, 2),
  FailedInlineParsing = get_log_id(CORE_GRP, LogSubGrp::Element as u8, LogSubSubGrp::General as u8, LogKind::Error, 3),
}

#[derive(Debug)]
pub enum AtomicErrLogId {
  InvalidHeadingLvl = get_log_id(CORE_GRP, LogSubGrp::Element as u8, LogSubSubGrp::Atomic as u8, LogKind::Error, 0),
}

#[derive(Debug)]
pub enum EnclosedErrLogId {
  FailedParsing = get_log_id(CORE_GRP, LogSubGrp::Element as u8, LogSubSubGrp::Enclosed as u8, LogKind::Error, 0),
}

#[derive(Debug)]
pub enum MetaDataErrLogId {
  FailedReadingFile = get_log_id(CORE_GRP, LogSubGrp::Element as u8, LogSubSubGrp::MetaData as u8, LogKind::Error, 0),
}
