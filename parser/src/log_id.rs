//! Defined parser log-ids

use logid::{ErrLogId, InfoLogId, WarnLogId};

/// Parser error log-ids
#[derive(Debug, Clone, ErrLogId)]
pub enum ParserError {
    /// Log-id denoting that no Unimarkup element was detected
    NoUnimarkupDetected,
}

/// Parser warning log-ids
#[derive(Debug, Clone, WarnLogId)]
pub enum ParserWarning {
    /// Log-id denoting an unsupported Unimarkup block
    UnsupportedBlock,
}

/// Parser info log-ids
#[derive(Debug, Clone, InfoLogId)]
pub enum MainParserInfo {
    /// Log-id denoting that the main parser is being initialized
    StartInitializing,

    /// Log-id denoting that the main parser was initialized
    Initialized,
}

impl std::fmt::Display for MainParserInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainParserInfo::StartInitializing => write!(f, "Initializing MainParser"),
            MainParserInfo::Initialized => write!(f, "MainParser initialized"),
        }
    }
}
