//! Log-id handling functionality for unimarkup-rs

/// Log-id main group number for the core crate
pub const CORE_GRP: u8 = 0;

// Note: LogSubGrps become obsolete after [https://github.com/rust-lang/rust/issues/60553]() gets into stable

/// Log-id sub group for the core crate
pub enum LogSubGrp {
    /// General log-id sub group
    General = 0,
    /// Frontend log-id sub group
    Frontend = 1,
    /// Middleend log-id sub group
    Middleend = 2,
    /// Backend log-id sub group
    Backend = 3,
    /// Element log-id sub group
    Element = 4,
    /// Config log-id sub group
    Config = 5,
    /// Security log-id sub group
    Security = 6,
}

enum SubSubGrp {
    General = 0,
}

/// Log-ids for config errors
pub enum ConfigErrLogId {
    /// Log-id denoting an invalid file in the config
    InvalidFile = get_log_id(
        CORE_GRP,
        LogSubGrp::Config as u8,
        SubSubGrp::General as u8,
        LogKind::Error,
        0,
    ),
    /// Log-id denoting an invalid path in the config
    InvalidPath = get_log_id(
        CORE_GRP,
        LogSubGrp::Config as u8,
        SubSubGrp::General as u8,
        LogKind::Error,
        1,
    ),
    /// Log-id denoting an invalid config
    InvalidConfig = get_log_id(
        CORE_GRP,
        LogSubGrp::Config as u8,
        SubSubGrp::General as u8,
        LogKind::Error,
        2,
    ),
}

//------------------------------------------------------------------------------------------------------------------
//                                        General Log functionality
//------------------------------------------------------------------------------------------------------------------

/// Type to represent a LogId.
/// Note: Wrapper of `isize` for easier `id <=> enum` conversion.
pub type LogId = isize;

/// Represents a invalid log-id
pub const NO_LOG_ID: LogId = 0;
/// Bit shift in the log-id to place the LogKind value
pub const LOG_KIND_SHIFT: i32 = 16;

/// Log kind a log-id can represent.
#[derive(Debug)]
pub enum LogKind {
    /// Log-id error kind
    Error = 3,
    /// Log-id warning kind
    Warn = 2,
    /// Log-id info kind
    Info = 1,
    /// Log-id debug kind
    Debug = 0,
}

/// Trait for [`LogId`] functionality
pub trait SetLog {
    /// Set a log message for a [`LogId`]
    ///
    /// # Arguments
    ///
    /// * `msg` - log message that is logged for this log-id
    /// * `filename` - name of the source file where the log is set (Note: use `file!()`)
    /// * `line_nr` - line number where the log is set (Note: use `line!()`)
    fn set_log(self, msg: &str, filename: &str, line_nr: u32) -> Self;

    /// Add an info log message for this log-id
    fn add_info(self, msg: &str) -> Self;

    /// Add an debug log message for this log-id
    fn add_debug(self, msg: &str) -> Self;

    /// Add an trace log message for this log-id
    fn add_trace(self, msg: &str) -> Self;

    /// Get the [`LogKind`] of this log-id
    fn get_kind(self) -> LogKind;
}

impl SetLog for LogId {
    fn set_log(self, msg: &str, filename: &str, line_nr: u32) -> LogId {
        let kind = self.get_kind();

        match kind {
            LogKind::Error => log::error!("{}: {}", self, msg),
            LogKind::Warn => log::warn!("{}: {}", self, msg),
            LogKind::Info => log::info!("{}: {}", self, msg),
            LogKind::Debug => log::debug!("{}: {}", self, msg),
        }

        log::trace!(
            "{}: Occured in file \"{}\" at line: {}",
            self,
            filename,
            line_nr
        );
        self
    }

    fn add_info(self, msg: &str) -> LogId {
        log::info!("{}(additional info): {}", self, msg);
        self
    }

    fn add_debug(self, msg: &str) -> LogId {
        log::debug!("{}(additional info): {}", self, msg);
        self
    }

    fn add_trace(self, msg: &str) -> LogId {
        log::trace!("{}(additional info): {}", self, msg);
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
            log::trace!("{}: Invalid kind: '{}'", self, kind);
            LogKind::Error
        }
    }
}

/// Returns a 32-bit log-id that is used to identify a log message across a project.
/// The log-id is a unique signed integer value that is identified by bit shifting given group numbers and log kind.
///
/// The log-id bits are represented as follows:
///
/// `32-30 bit = main group | 29 - 25 bit = sub group | 24 - 19 bit = sub sub group | 16 - 17 bit = log kind | remaining 16 bit = local number`
///
/// # Arguments
///
/// * `main_grp` - main group the log-id is assigned to (possible range: 0 .. 2^3 - 1)
/// * `sub_grp` - sub group the log-id is assigned to (possible range: 0 .. 2^5 - 1)
/// * `sub_sub_grp` - sub sub group the log-id is assigned to (possible range: 0 .. 2^6 - 1)
/// * `log_kind` - the ['LogKind'] of the log-id
/// * `local_nr` - the local number of the log-id (possible range: 0 .. 2^16 - 1)
///
/// # Example
///
/// ~~~
/// use unimarkup_core::log_id::{get_log_id, LogKind};
///
/// assert_eq!(get_log_id(0, 0, 0, LogKind::Debug, 1), 1);
/// assert_eq!(get_log_id(1, 0, 0, LogKind::Error, 1), 537067521);
/// ~~~
pub const fn get_log_id(
    main_grp: u8,
    sub_grp: u8,
    sub_sub_grp: u8,
    log_kind: LogKind,
    local_nr: u16,
) -> LogId {
    let log_kind_number: i32 = log_kind as i32;

    // Id = 0 is not allowed
    //
    // TODO: needs unstable "panic!() in const fn" feature. Uncomment after feature is in stable
    //panic!((main_grp == 0) && (sub_grp == 0) && (sub_sub_grp == 0) && (log_kind_number == 0) && (local_nr == 0), "Log ID 0 is not allowed!");
    //panic!((main_grp >= 2^3) || (sub_grp >= 2^5) || (sub_sub_grp >= 2^6), "At least one log ID subrange is invalid.");

    (((main_grp as i32) << 29)
        + ((sub_grp as i32) << 24)
        + ((sub_sub_grp as i32) << 18)
        + (log_kind_number << LOG_KIND_SHIFT)
        + (local_nr as i32)) as LogId
}
