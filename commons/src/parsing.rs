//! Contains the [`Context`] struct used while parsing Unimarkup content.

/// Context to help with parsing Unimarkup content.
#[derive(Debug, Clone)]
pub struct Context {
    /// Flag to indicate that only escaped symbols and macros are allowed besides plain content.
    pub macros_only: bool,
    /// Flag to indicate that multiple contiguous spaces must not be combined.
    pub keep_spaces: bool,
}
