use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct ScanError {
    message: String,
    line: usize,
}

impl ScanError {
    pub fn new<A: Into<String>>(message: A, line: usize) -> Self {
        Self {
            message: message.into(),
            line,
        }
    }
}

impl Display for ScanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[line {}] error: {}", self.line, self.message)
    }
}
