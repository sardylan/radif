use crate::data::AdifData;
use crate::error::AdifError;
use adif_macros::{AdifData, AutoDisplay, AutoTestEnum};

#[derive(Debug, Clone, PartialEq, Eq, AutoDisplay, AdifData, AutoTestEnum)]
pub enum Region {
    #[adif("NONE")]
    None,
    #[adif("IV")]
    ITUVienna,
    #[adif("AI")]
    AfricanItaly,
    #[adif("SY")]
    Sicily,
    #[adif("BI")]
    BearIsland,
    #[adif("SI")]
    ShetlandIslands,
    #[adif("KO")]
    Kosovo,
    #[adif("ET")]
    EuropeanTurkey,
}
