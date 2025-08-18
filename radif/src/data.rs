use crate::result;
use std::fmt::{Debug, Display};

pub trait AdifData: Debug + Clone + PartialEq + Display {
    fn serialize(&self) -> String;
    fn deserialize(value: &str) -> result::Result<Self>
    where
        Self: Sized;
}
