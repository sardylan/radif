use crate::data::AdifData;
use crate::error::AdifError;
use adif_macros::{AdifData, AutoDisplay, AutoTestEnum};

#[derive(Debug, Clone, PartialEq, Eq, AutoDisplay, AdifData, AutoTestEnum)]
pub enum QslVia {
    #[adif("B")]
    Bureau,
    #[adif("D")]
    Direct,
    #[adif("E")]
    Electronic,
    #[adif("M")]
    Manager,
}
