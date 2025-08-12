use crate::data::AdifData;
use crate::error::AdifError;
use adif_macros::{AdifData, AutoDisplay, AutoTestEnum};

#[derive(Debug, Clone, PartialEq, Eq, AutoDisplay, AdifData, AutoTestEnum)]
pub enum Continent {
    #[adif("NA")]
    NorthAmerica,
    #[adif("SA")]
    SouthAmerica,
    #[adif("EU")]
    Europe,
    #[adif("AF")]
    Africa,
    #[adif("OC")]
    Oceana,
    #[adif("AS")]
    Asia,
    #[adif("AN")]
    Antarctica,
}
