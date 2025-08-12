use crate::result;
use std::fmt::Display;

pub trait AdifData: Display {
    fn serialize(&self) -> String;
    fn deserialize(value: &str) -> result::Result<Self>
    where
        Self: Sized;
}
