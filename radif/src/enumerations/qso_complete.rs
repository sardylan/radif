use crate::data::AdifData;
use crate::error::AdifError;
use adif_macros::{AdifData, AutoDisplay, AutoTestEnum};

#[derive(Debug, Clone, PartialEq, Eq, AutoDisplay, AdifData, AutoTestEnum)]
pub enum QsoComplete {
    #[adif("Y")]
    Yes,
    #[adif("N")]
    No,
    #[adif("NIL")]
    NotHeard,
    #[adif("?")]
    Uncertain,
}
