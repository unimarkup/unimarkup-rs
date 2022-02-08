use crate::log_id::{get_log_id, CORE_GRP, LogSubGrp, LogKind};


enum LogSubSubGrp {
  General = 0,
  Loader = 1,
  Renderer = 2,
  Inline = 3,
}

pub enum LoaderErrLogId {
  InvalidElementType = get_log_id(CORE_GRP, LogSubGrp::Backend as u8, LogSubSubGrp::Loader as u8, LogKind::Error, 0),
}

pub enum InlineErrLogId {
  NoInlineDetected = get_log_id(CORE_GRP, LogSubGrp::Backend as u8, LogSubSubGrp::Inline as u8, LogKind::Error, 0),
}
