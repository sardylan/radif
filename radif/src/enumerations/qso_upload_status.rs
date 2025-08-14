use crate::data::AdifData;
use crate::error::AdifError;
use radif_macros::{AdifData, AutoDisplay, AutoTestEnum};

#[derive(Debug, Clone, PartialEq, Eq, AutoDisplay, AdifData, AutoTestEnum)]
pub enum QsoUploadStatus {
    #[adif("Y")]
    Yes,
    #[adif("N")]
    No,
    #[adif("M")]
    Modified,
}
