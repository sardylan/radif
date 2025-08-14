use crate::data::AdifData;
use crate::field::Field;
use crate::fields::data::DataValue;
use crate::fields::qso::QSOFieldName;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct QSOField {
    name: QSOFieldName,
    value: DataValue,
}

impl Field for QSOField {
    type FN = QSOFieldName;

    fn get_name(&self) -> &Self::FN {
        &self.name
    }

    fn get_value(&self) -> &DataValue {
        &self.value
    }

    fn new(name: Self::FN, value: DataValue) -> Self {
        Self { name, value }
    }
}

impl Display for QSOField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::AdifData;
    use crate::field::Field;
    use crate::fields::data::DataValue;
    use crate::fields::qso::QSOFieldName;
    use crate::qso::QSOField;

    #[test]
    fn test_valid() {
        let field = QSOField::new(QSOFieldName::CALL, DataValue::String("IS0GVH".to_string()));
        assert_eq!(field.serialize(), "<CALL:6>IS0GVH");
    }
}
