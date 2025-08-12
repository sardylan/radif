use crate::data::AdifData;
use crate::field::Field;
use crate::fields::data::DataValue;
use crate::fields::header::HeaderFieldName;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct HeaderField {
    name: HeaderFieldName,
    value: DataValue,
    number: Option<u32>,
}

impl Field for HeaderField {
    type FN = HeaderFieldName;

    fn get_name(&self) -> &Self::FN {
        &self.name
    }

    fn get_value(&self) -> &DataValue {
        &self.value
    }

    fn new(name: Self::FN, value: DataValue) -> Self {
        match name {
            HeaderFieldName::USERDEF(n) => Self {
                name,
                value,
                number: Some(n),
            },
            HeaderFieldName::APP => Self {
                name,
                value,
                number: None,
            },
            _ => Self {
                name,
                value,
                number: None,
            },
        }
    }
}

impl Display for HeaderField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::AdifData;
    use crate::field::Field;
    use crate::fields::data::DataValue;
    use crate::fields::header::HeaderFieldName;
    use crate::header::HeaderField;

    #[test]
    fn test_valid() {
        let field = HeaderField::new(
            HeaderFieldName::PROGRAMID,
            DataValue::String("Test".to_string()),
        );
        assert_eq!(field.serialize(), "<PROGRAMID:4>Test");
    }
}
