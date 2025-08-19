use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AdifError {
    SerializeError(String),
    DeserializeError(String),
    GenericError(String),
}

impl Display for AdifError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdifError::SerializeError(message) => write!(f, "SerializeError: {}", message),
            AdifError::DeserializeError(message) => write!(f, "DeserializeError: {}", message),
            AdifError::GenericError(message) => write!(f, "GenericError: {}", message),
        }
    }
}

impl Error for AdifError {}

// impl From<> for AdifError {}