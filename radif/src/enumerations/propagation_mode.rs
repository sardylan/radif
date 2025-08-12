use crate::data::AdifData;
use crate::error::AdifError;
use adif_macros::{AdifData, AutoDisplay, AutoTestEnum};

#[derive(Debug, Clone, PartialEq, Eq, AutoDisplay, AdifData, AutoTestEnum)]
pub enum PropagationMode {
    #[adif("AS")]
    AircraftScatter,
    #[adif("AUE")]
    AuroraE,
    #[adif("AUR")]
    Aurora,
    #[adif("BS")]
    BackScatter,
    #[adif("ECH")]
    EchoLink,
    #[adif("EME")]
    EarthMoonEarth,
    #[adif("ES")]
    SporadicE,
    #[adif("F2")]
    F2Reflection,
    #[adif("FAI")]
    FieldAlignedIrregularities,
    #[adif("GWAVE")]
    GroundWave,
    #[adif("INTERNET")]
    InternetAssisted,
    #[adif("ION")]
    Ionoscatter,
    #[adif("IRL")]
    IRLP,
    #[adif("LOS")]
    LineOfSight,
    #[adif("MS")]
    MeteorScatter,
    #[adif("RPT")]
    Repeater,
    #[adif("RS")]
    RainScatter,
    #[adif("SAT")]
    Satellite,
    #[adif("TEP")]
    TransEquatorial,
    #[adif("TR")]
    TroposphericDucting,
}
