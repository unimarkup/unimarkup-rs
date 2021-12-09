use core::fmt;

#[derive(Debug)]
pub struct BackendError {
    message: String,
}

impl BackendError {
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
