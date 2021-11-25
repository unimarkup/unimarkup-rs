use core::fmt;

#[derive(Debug)]
pub struct BackErr {
    message: String,
}

impl BackErr {
    pub fn new(msg: impl Into<String>) -> Self {
        BackErr {
            message: msg.into(),
        }
    }
}

impl fmt::Display for BackErr {
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
