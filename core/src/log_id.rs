use log::{debug, info, warn, error, trace};



pub const CORE_GRP: u8 = 0;

// Note: LogSubGrps become obsolete after [https://github.com/rust-lang/rust/issues/60553]() gets into stable

pub enum LogSubGrp {
  General = 0,
  Frontend = 1,
  Middleend = 2,
  Backend = 3,
  Element = 4,
  Config = 5,
}

enum SubSubGrp {
  General = 0,
}

pub enum ConfigErrLogId {
  InvalidFile = get_log_id(CORE_GRP, LogSubGrp::Config as u8, SubSubGrp::General as u8, LogKind::Error, 0),
  InvalidPath = get_log_id(CORE_GRP, LogSubGrp::Config as u8, SubSubGrp::General as u8, LogKind::Error, 1),
  InvalidConfig = get_log_id(CORE_GRP, LogSubGrp::Config as u8, SubSubGrp::General as u8, LogKind::Error, 2),
}


//------------------------------------------------------------------------------------------------------------------
//                                        General Log functionality
//------------------------------------------------------------------------------------------------------------------

pub type LogId = isize;


pub const NO_LOG_ID: LogId = 0;
pub const LOG_KIND_SHIFT: i32 = 16;

#[derive(Debug)]
pub enum LogKind {
  Error = 3,
  Warn = 2,
  Info = 1,
  Debug = 0,
}

pub trait SetLog {
  fn set_log(self, msg: &str, filename: &str, line_nr: u32) -> Self;
  fn add_info(self, msg: &str) -> Self;
  fn add_debug(self, msg: &str) -> Self;
  fn add_trace(self, msg: &str) -> Self;
  fn get_kind(self) -> LogKind;
}

impl SetLog for LogId {
  fn set_log(self, msg: &str, filename: &str, line_nr: u32) -> LogId {
    let kind = self.get_kind();

    match kind {
        LogKind::Error => error!("{}: {}", self, msg),
        LogKind::Warn => warn!("{}: {}", self, msg),
        LogKind::Info => info!("{}: {}", self, msg),
        LogKind::Debug => debug!("{}: {}", self, msg),
    }
  
    trace!("{}: Occured in file \"{}\" at line: {}", self, filename, line_nr);
    self
  }
  
  fn add_info(self, msg: &str) -> LogId {
    info!("{}(additional info): {}", self, msg);
    self
  }

  fn add_debug(self, msg: &str) -> LogId {
    debug!("{}(additional info): {}", self, msg);
    self
  }

  fn add_trace(self, msg: &str) -> LogId {
    trace!("{}(additional info): {}", self, msg);
    self
  }

  fn get_kind(self) -> LogKind {
    // get LogKind bits
    let kind = (self >> LOG_KIND_SHIFT) & 3;

    if kind == (LogKind::Error as isize) {
      LogKind::Error
    } else if kind == (LogKind::Warn as isize) {
      LogKind::Warn
    } else if kind == (LogKind::Info as isize) {
      LogKind::Info
    } else if kind == (LogKind::Debug as isize) {
      LogKind::Debug
    } else {
      trace!("{}: Invalid kind: '{}'", self, kind);
      LogKind::Error
    }
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
