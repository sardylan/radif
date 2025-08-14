use crate::data::AdifData;
use crate::error::AdifError;
use radif_macros::{AdifData, AutoDisplay, AutoTestEnum};

#[derive(Debug, Clone, PartialEq, Eq, AutoDisplay, AdifData, AutoTestEnum)]
pub enum QslSent {
    #[adif("Y")]
    Yes,
    #[adif("N")]
    No,
    #[adif("R")]
    Requested,
    #[adif("Q")]
    Queued,
    #[adif("I")]
    Invalid,
}
