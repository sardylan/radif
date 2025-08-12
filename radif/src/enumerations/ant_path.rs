use crate::data::AdifData;
use crate::error::AdifError;
use adif_macros::{AdifData, AutoDisplay, AutoTestEnum};

#[derive(Debug, Clone, PartialEq, Eq, AutoDisplay, AdifData, AutoTestEnum)]
pub enum AntPath {
    #[adif("G")]
    Grayline,
    #[adif("O")]
    Other,
    #[adif("S")]
    ShortPath,
    #[adif("L")]
    LongPath,
}
