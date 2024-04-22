use core::fmt;
use std::{backtrace::Backtrace, error::Error};

#[derive(Debug)]
pub struct OxideError {
    message: String,
    backtrace: Backtrace,
}

impl OxideError {
    pub fn new(msg: &str) -> Box<OxideError> {
        Box::new(OxideError {
            message: msg.to_owned(),
            
            backtrace: Backtrace::capture()
        })
    }
}

impl Error for OxideError {}

impl fmt::Display for OxideError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.message,self.backtrace)
    }
}

pub type OxideResult<T> = Result<T, Box<dyn Error>>;

