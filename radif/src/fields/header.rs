use crate::data::AdifData;
use crate::error::AdifError::DeserializeError;
use crate::field::FieldName;
use crate::fields::data::DataType;
use crate::result;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum HeaderFieldName {
    ADIF_VER,
    CREATED_TIMESTAMP,
    PROGRAMID,
    PROGRAMVERSION,
    APP, // # TODO: Application specific field, can be used for any application-specific data
    USERDEF(u32),
    EOH,
}

impl AdifData for HeaderFieldName {
    fn serialize(&self) -> String {
        match self {
            HeaderFieldName::ADIF_VER => "ADIF_VER".to_string(),
            HeaderFieldName::CREATED_TIMESTAMP => "CREATED_TIMESTAMP".to_string(),
            HeaderFieldName::PROGRAMID => "PROGRAMID".to_string(),
            HeaderFieldName::PROGRAMVERSION => "PROGRAMVERSION".to_string(),
            HeaderFieldName::APP => "APP".to_string(), // TODO: Application specific field
            HeaderFieldName::USERDEF(value) => format!("USERDEF{}", value),
            HeaderFieldName::EOH => "EOH".to_string(),
        }
    }

    fn deserialize(value: &str) -> result::Result<Self>
    where
        Self: Sized,
    {
        match value {
            "ADIF_VER" => Ok(HeaderFieldName::ADIF_VER),
            "CREATED_TIMESTAMP" => Ok(HeaderFieldName::CREATED_TIMESTAMP),
            "PROGRAMID" => Ok(HeaderFieldName::PROGRAMID),
            "PROGRAMVERSION" => Ok(HeaderFieldName::PROGRAMVERSION),
            "APP" => Ok(HeaderFieldName::APP), // TODO: Application specific field
            userdef if userdef.starts_with("USERDEF") => match userdef[7..].trim().parse::<u32>() {
                Ok(num) => Ok(HeaderFieldName::USERDEF(num)),
                Err(_) => Err(DeserializeError("Invalid USERDEF value".to_string())),
            },
            "EOH" => Ok(HeaderFieldName::EOH),
            &_ => Err(DeserializeError("Unknown header field name".to_string())),
        }
    }
}

impl FieldName for HeaderFieldName {
    fn get_data_type(&self) -> DataType {
        match self {
            HeaderFieldName::ADIF_VER => DataType::String,
            HeaderFieldName::CREATED_TIMESTAMP => DataType::String,
            HeaderFieldName::PROGRAMID => DataType::String,
            HeaderFieldName::PROGRAMVERSION => DataType::String,
            HeaderFieldName::APP => DataType::String, // TODO: Application specific field
            HeaderFieldName::USERDEF(_) => DataType::String,
            HeaderFieldName::EOH => DataType::Null,
        }
    }
}

impl Display for HeaderFieldName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_field_name_serialize() {
        assert_eq!(HeaderFieldName::ADIF_VER.serialize(), "ADIF_VER");
        assert_eq!(
            HeaderFieldName::CREATED_TIMESTAMP.serialize(),
            "CREATED_TIMESTAMP"
        );
        assert_eq!(HeaderFieldName::PROGRAMID.serialize(), "PROGRAMID");
        assert_eq!(
            HeaderFieldName::PROGRAMVERSION.serialize(),
            "PROGRAMVERSION"
        );
        assert_eq!(HeaderFieldName::USERDEF(42).serialize(), "USERDEF42");
        assert_eq!(HeaderFieldName::EOH.serialize(), "EOH");
    }

    #[test]
    fn test_header_field_name_deserialize() {
        assert_eq!(
            HeaderFieldName::deserialize("ADIF_VER").unwrap(),
            HeaderFieldName::ADIF_VER
        );
        assert_eq!(
            HeaderFieldName::deserialize("CREATED_TIMESTAMP").unwrap(),
            HeaderFieldName::CREATED_TIMESTAMP
        );
        assert_eq!(
            HeaderFieldName::deserialize("PROGRAMID").unwrap(),
            HeaderFieldName::PROGRAMID
        );
        assert_eq!(
            HeaderFieldName::deserialize("PROGRAMVERSION").unwrap(),
            HeaderFieldName::PROGRAMVERSION
        );
        assert_eq!(
            HeaderFieldName::deserialize("USERDEF42").unwrap(),
            HeaderFieldName::USERDEF(42)
        );
        assert_eq!(
            HeaderFieldName::deserialize("EOH").unwrap(),
            HeaderFieldName::EOH
        );
        assert!(HeaderFieldName::deserialize("INVALID").is_err());
    }
}
