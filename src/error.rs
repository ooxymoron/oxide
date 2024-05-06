use core::fmt;
use std::{error::Error};

#[derive(Debug)]
pub struct OxideError {
    message: String,
}

impl OxideError {
    pub fn new(msg: &str) -> Box<OxideError> {
        Box::new(OxideError {
            message: msg.to_owned(),
        })
    }
}

impl Error for OxideError {}

impl fmt::Display for OxideError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub type OxideResult<T> = Result<T, Box<dyn Error>>;

