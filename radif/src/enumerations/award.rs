use crate::data::AdifData;
use crate::error::AdifError;
use radif_macros::{AdifData, AutoDisplay, AutoTestEnum};

#[derive(Debug, Clone, PartialEq, Eq, AutoDisplay, AdifData, AutoTestEnum)]
#[allow(non_camel_case_types)]
pub enum Award {
    #[adif("AJA")]
    AJA,
    #[adif("CQDX")]
    CQDX,
    #[adif("CQDXFIELD")]
    CQDXFIELD,
    #[adif("CQWAZ_MIXED")]
    CQWAZ_MIXED,
    #[adif("CQWAZ_CW")]
    CQWAZ_CW,
    #[adif("CQWAZ_PHONE")]
    CQWAZ_PHONE,
    #[adif("CQWAZ_RTTY")]
    CQWAZ_RTTY,
    #[adif("CQWAZ_160m")]
    CQWAZ_160m,
    #[adif("CQWPX")]
    CQWPX,
    #[adif("DARC_DOK")]
    DARC_DOK,
    #[adif("DXCC")]
    DXCC,
    #[adif("DXCC_MIXED")]
    DXCC_MIXED,
    #[adif("DXCC_CW")]
    DXCC_CW,
    #[adif("DXCC_PHONE")]
    DXCC_PHONE,
    #[adif("DXCC_RTTY")]
    DXCC_RTTY,
    #[adif("IOTA")]
    IOTA,
    #[adif("JCC")]
    JCC,
    #[adif("JCG")]
    JCG,
    #[adif("MARATHON")]
    MARATHON,
    #[adif("RDA")]
    RDA,
    #[adif("WAB")]
    WAB,
    #[adif("WAC")]
    WAC,
    #[adif("WAE")]
    WAE,
    #[adif("WAIP")]
    WAIP,
    #[adif("WAJA")]
    WAJA,
    #[adif("WAS")]
    WAS,
    #[adif("WAZ")]
    WAZ,
    #[adif("USACA")]
    USACA,
    #[adif("VUCC")]
    VUCC,
}
