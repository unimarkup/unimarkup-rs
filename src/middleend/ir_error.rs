use std::fmt;

#[derive(Debug)]
pub struct IrError {
    pub tablename: String,
    pub column: String,
    pub message: String,
}

impl fmt::Display for IrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Error in communication with IR."))?;

        f.write_fmt(format_args!(
            "\nError occured in column {} of table {}.",
            self.column, self.tablename
        ))?;

        let prefix = "Message: ";

        let msg: String = self
            .message
            .lines()
            .enumerate()
            .map(|(i, line)| {
                if i > 0 {
                    " ".repeat(prefix.len()) + line
                } else {
                    String::from(line)
                }
            })
            .collect();

        f.write_fmt(format_args!("{}{}", prefix, msg))?;

        Ok(())
    }
}
