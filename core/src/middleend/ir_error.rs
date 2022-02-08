// use std::fmt;

// /// [`IrError`] is one of the possible Error variants of [`UmError`].
// ///
// /// It's returned in case, where some operation fails in the [`middleend`] module
// /// of [`unimarkup-rs`], e.g. when communication with the IR fails.
// ///
// /// [`UmError`]: crate::error::UmError
// /// [`middleend`]: crate::middleend
// #[derive(Debug)]
// pub struct IrError {
//     /// The name of the target table, when the error occured.
//     pub tablename: String,
//     /// The name of the target column in the table, when the error occured.
//     pub column: String,
//     /// Custom message to provide more information about the error.
//     pub message: String,
// }

// impl IrError {
//     /// Constructs a new IrError
//     ///
//     /// # Arguments
//     ///
//     /// * `tablename` - Name of the target table
//     /// * `column` - Name of the target column in the table
//     /// * `message` - Custom error message
//     pub fn new(
//         tablename: impl Into<String>,
//         column: impl Into<String>,
//         message: impl Into<String>,
//     ) -> Self {
//         Self {
//             tablename: tablename.into(),
//             column: column.into(),
//             message: message.into(),
//         }
//     }
// }

// impl fmt::Display for IrError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.write_fmt(format_args!("Error in communication with IR."))?;

//         f.write_fmt(format_args!(
//             "\nError occured in column {} of table {}.",
//             self.column, self.tablename
//         ))?;

//         let prefix = "Message: ";

//         let msg: String = self
//             .message
//             .lines()
//             .enumerate()
//             .map(|(i, line)| {
//                 if i > 0 {
//                     " ".repeat(prefix.len()) + line
//                 } else {
//                     String::from(line)
//                 }
//             })
//             .collect();

//         f.write_fmt(format_args!("{}{}", prefix, msg))?;

//         Ok(())
//     }
// }
