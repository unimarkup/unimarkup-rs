

pub const CORE_GRP: u8 = 0;

pub enum LogSubGrp {
  General = 0,
  Frontend = 1,
  Middleend = 2,
  Backend = 3,
  Elements = 4,
}

enum SubSubGrp {
  General = 0,
  Config = 1,
}

pub enum ConfigErrLogId {
  InvalidFile = get_log_id(CORE_GRP, LogSubGrp::General as u8, SubSubGrp::Config as u8, LogKind::Error, 0),
  InvalidPath = get_log_id(CORE_GRP, LogSubGrp::General as u8, SubSubGrp::Config as u8, LogKind::Error, 1),
  InvalidConfig = get_log_id(CORE_GRP, LogSubGrp::General as u8, SubSubGrp::Config as u8, LogKind::Error, 2),
}


//------------------------------------------------------------------------------------------------------------------
//                                        General Log functionality
//------------------------------------------------------------------------------------------------------------------

pub type LogId = isize;


pub const NO_LOG_ID: LogId = 0;
pub const LOG_KIND_SHIFT: i32 = 16;

pub enum LogKind {
  Error = 3,
  Warn = 2,
  Info = 1,
  Debug = 0,
}

pub trait SetLog {
  fn set_log(self, msg: &str, filename: &str, line_nr: u32) -> Self;
  fn add_to_log(self, msg: &str) -> Self;
}

impl SetLog for LogId {
  fn set_log(self, msg: &str, filename: &str, line_nr: u32) -> LogId {
    // get LogKind bits
    let kind = (self >> LOG_KIND_SHIFT) & 3;

    if kind == (LogKind::Error as isize) {
      log::error!("{}: {}", self, msg)
    } else if kind == (LogKind::Warn as isize) {
      log::warn!("{}: {}", self, msg)
    } else if kind == (LogKind::Info as isize) {
      log::info!("{}: {}", self, msg)
    } else if kind == (LogKind::Debug as isize) {
      log::debug!("{}: {}", self, msg)
    } else {
      log::trace!("{}: Invalid kind: '{}'", self, kind)
    }
  
    log::trace!("{}: Occured in file `{}` at line = {}", self, filename, line_nr);
    self
  }
  
  fn add_to_log(self, msg: &str) -> LogId {
    log::debug!("{} (additional info): {}", self, msg);
    self
  }
}


pub const fn get_log_id(main_grp: u8, sub_grp: u8, sub_sub_grp: u8, log_kind: LogKind, local_nr: u16) -> LogId {
  let log_kind_number: i32 = log_kind as i32;
  
  // Id = 0 is not allowed
  //
  // TODO: needs unstable "panic!() in const fn" feature. Uncomment after feature is in stable
  //assert!((main_grp == 0) && (sub_grp == 0) && (sub_sub_grp == 0) && (log_kind_number == 0) && (local_nr == 0), "Log ID 0 is not allowed!");
  //assert!((main_grp >= 2^3) || (sub_grp >= 2^5) || (sub_sub_grp >= 2^6), "At least one log ID subrange is invalid.");
  
  (((main_grp as i32) << 29) + ((sub_grp as i32) << 24) + ((sub_sub_grp as i32) << 18) + (log_kind_number << LOG_KIND_SHIFT) + (local_nr as i32)) as LogId
}
