use crate::data::AdifData;
use crate::error::AdifError;
use crate::result;
use adif_macros::{AdifData, AutoDisplay};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, AdifData, AutoDisplay)]
pub enum Sponsor {
    ADIF,
    ARI,
    ARRL,
    CQ,
    DARC,
    EQSL,
    IARU,
    JARL,
    RSGB,
    TAG,
    WABAG,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SponsoredAward {
    pub sponsor: Sponsor,
    pub program: String,
    pub award: String,
}

impl AdifData for SponsoredAward {
    fn serialize(&self) -> String {
        format!(
            "{}_{}_{}",
            self.sponsor.serialize(),
            self.program,
            self.award
        )
    }

    fn deserialize(value: &str) -> result::Result<Self>
    where
        Self: Sized,
    {
        let parts: Vec<&str> = value.split('_').collect();
        if parts.len() != 3 {
            return Err(AdifError::DeserializeError(
                "SponsoredAward must have exactly three parts".to_string(),
            ));
        }

        let sponsor = Sponsor::deserialize(parts[0])?;
        let program = parts[1].to_string();
        let award = parts[2].to_string();

        Ok(SponsoredAward {
            sponsor,
            program,
            award,
        })
    }
}

impl Display for SponsoredAward {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sponsored_award_serialize() {
        let award = SponsoredAward {
            sponsor: Sponsor::ARRL,
            program: "DXCC".to_string(),
            award: "Mixed".to_string(),
        };
        assert_eq!(award.serialize(), "ARRL_DXCC_Mixed");
    }

    #[test]
    fn test_sponsored_award_deserialize() {
        let value = "CQ_WAS_Single";
        let award = SponsoredAward::deserialize(value).unwrap();
        assert_eq!(award.sponsor, Sponsor::CQ);
        assert_eq!(award.program, "WAS");
        assert_eq!(award.award, "Single");
    }

    #[test]
    fn test_sponsored_award_deserialize_error() {
        let value = "InvalidData";
        let result = SponsoredAward::deserialize(value);
        assert!(result.is_err());
    }
}
