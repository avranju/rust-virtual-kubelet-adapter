use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, Clone)]
pub enum ErrorType {
    Unknown,
    NotFound,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub etype: ErrorType,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Error {
            message: String::from(msg),
            etype: ErrorType::Unknown,
        }
    }

    pub fn not_found(msg: &str) -> Self {
        Error {
            message: String::from(msg),
            etype: ErrorType::NotFound,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}
