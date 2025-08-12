use crate::data::AdifData;
use crate::error::AdifError;
use adif_macros::{AdifData, AutoDisplay, AutoTestEnum};

#[derive(Debug, Clone, PartialEq, Eq, AutoDisplay, AdifData, AutoTestEnum)]
pub enum MorseKeyType {
    #[adif("SK")]
    StraightKey,
    #[adif("SS")]
    Sideswiper,
    #[adif("BUG")]
    MechanicalSemiAutomaticKeyerOrBug,
    #[adif("FAB")]
    MechanicalFullyAutomaticKeyerOrBug,
    #[adif("SP")]
    SinglePaddle,
    #[adif("DP")]
    DualPaddle,
    #[adif("CPU")]
    ComputerDriven,
}
