use core::fmt;

use crate::frontend::SyntaxError;
use crate::middleend::IrError;

pub enum UmError {
    Syntax(SyntaxError),
    Ir(IrError),
}

impl From<SyntaxError> for UmError {
    fn from(syntax_error: SyntaxError) -> Self {
        Self::Syntax(syntax_error)
    }
}

impl From<IrError> for UmError {
    fn from(ir_error: IrError) -> Self {
        Self::Ir(ir_error)
    }
}

impl fmt::Display for UmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UmError::Syntax(err) => err.fmt(f),
            UmError::Ir(err) => err.fmt(f),
        }
    }
}
