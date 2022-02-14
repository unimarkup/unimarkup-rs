//! Defines log-ids for the element section

use crate::log_id::{get_log_id, CORE_GRP, LogSubGrp, LogKind};


enum LogSubSubGrp {
  General = 0,
  Atomic = 1,
  Enclosed = 2,
  MetaData = 3,
}

/// General error log-ids for the element section
#[derive(Debug)]
pub enum GeneralErrLogId {
  /// Log-id denoting an invalid attribute
  InvalidAttribute = get_log_id(CORE_GRP, LogSubGrp::Element as u8, LogSubSubGrp::General as u8, LogKind::Error, 0),
  /// Log-id denoting an invalid element type
  InvalidElementType = get_log_id(CORE_GRP, LogSubGrp::Element as u8, LogSubSubGrp::General as u8, LogKind::Error, 1),
  /// Log-id denoting a failed block creation
  FailedBlockCreation = get_log_id(CORE_GRP, LogSubGrp::Element as u8, LogSubSubGrp::General as u8, LogKind::Error, 2),
  /// Log-id denoting failed inline parsing
  FailedInlineParsing = get_log_id(CORE_GRP, LogSubGrp::Element as u8, LogSubSubGrp::General as u8, LogKind::Error, 3),
}

/// Atomic error log-ids for the element section
#[derive(Debug)]
pub enum AtomicErrLogId {
  /// Log-id denoting an invalid heading level
  InvalidHeadingLvl = get_log_id(CORE_GRP, LogSubGrp::Element as u8, LogSubSubGrp::Atomic as u8, LogKind::Error, 0),
}

/// Enclosed error log-ids for the element section
#[derive(Debug)]
pub enum EnclosedErrLogId {
  /// Log-id denoting failed parsing
  FailedParsing = get_log_id(CORE_GRP, LogSubGrp::Element as u8, LogSubSubGrp::Enclosed as u8, LogKind::Error, 0),
}

/// MetaData error log-ids for the element section
#[derive(Debug)]
pub enum MetaDataErrLogId {
  /// Log-id denoting a fail while reading a file
  FailedReadingFile = get_log_id(CORE_GRP, LogSubGrp::Element as u8, LogSubSubGrp::MetaData as u8, LogKind::Error, 0),
}
