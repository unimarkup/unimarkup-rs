use core::fmt;

/// [`BackendError`] is one of the possible Error variants of [`UmError`].
///
/// It's returned in case where some operations fails in [`backend`] module
/// of the [`unimarkup-rs`], i.e. when (re)constructing UnimarkupBlock from IR,
/// or trying to render one of the provided output formats.
///
/// [`UmError`]: crate::um_error::UmError
/// [`backend`]: crate::backend
#[derive(Debug)]
pub struct BackendError {
    message: String,
}

impl BackendError {
    /// Creates a new instance of [`BackendError`] with the given message
    pub fn new(msg: impl Into<String>) -> Self {
        BackendError {
            message: msg.into(),
        }
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = "Error: ";

        let msg: String = self
            .message
            .lines()
            .enumerate()
            .map(|(i, line)| {
                if i > 0 {
                    " ".repeat(prefix.len()) + line
                } else {
                    line.to_string()
                }
            })
            .collect();

        f.write_fmt(format_args!("Error: {}", msg))?;

        Ok(())
    }
}
