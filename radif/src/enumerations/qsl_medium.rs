use crate::data::AdifData;
use crate::error::AdifError;
use radif_macros::{AdifData, AutoDisplay, AutoTestEnum};

#[derive(Debug, Clone, PartialEq, Eq, AutoDisplay, AdifData, AutoTestEnum)]
#[allow(non_camel_case_types)]
pub enum QslMedium {
    #[adif("CARD")]
    Card,
    #[adif("EQSL")]
    eQSL,
    #[adif("LOTW")]
    LoTW,
}
