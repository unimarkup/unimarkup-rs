//! Contains the [`Context`] struct used while parsing Unimarkup content.

/// Context to help with parsing Unimarkup inline content.
#[derive(Debug, Default, Clone)]
pub struct InlineContext {
    pub flags: InlineContextFlags,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InlineContextFlags {
    /// Flag to indicate that only escaped symbols and logic elements are allowed besides plain content.
    pub logic_only: bool,
    /// Flag to indicate that multiple contiguous whitespaces must not be combined.
    pub keep_whitespaces: bool,
    /// Flag to indicate that a newline must be explicitly kept, and not converted to one space.
    pub keep_newline: bool,
    /// Flag to indicate if implicit substitutions are allowed in the current context
    pub allow_implicits: bool,
}
