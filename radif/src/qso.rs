use crate::adif::AdifItem;
use crate::data::AdifData;
use crate::error::AdifError;
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

    fn get_name_end(&self) -> &Self::FN {
        &QSOFieldName::EOR
    }

    fn new(name: Self::FN, value: DataValue) -> Self {
        Self { name, value }
    }

    fn end() -> Self {
        Self {
            name: QSOFieldName::EOR,
            value: DataValue::Null(),
        }
    }
}

impl Display for QSOField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QSO {
    qso: Vec<QSOField>,
}

impl AdifItem for QSO {
    fn add_end_if_missing(&self) -> Self {
        self.qso
            .last()
            .map_or_else(|| true, |last| last.name != QSOFieldName::EOR)
            .then(|| Self {
                qso: self
                    .qso
                    .iter()
                    .cloned()
                    .chain(std::iter::once(QSOField::end()))
                    .collect(),
            })
            .unwrap_or_else(|| self.clone())
    }
}

impl Display for QSO {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

impl AdifData for QSO {
    fn serialize(&self) -> String {
        self.qso
            .iter()
            .map(QSOField::serialize)
            .collect::<Vec<String>>()
            .join("")
    }

    fn deserialize(_value: &str) -> crate::result::Result<Self>
    where
        Self: Sized,
    {
        Err(AdifError::DeserializeError(
            "QSO deserialization not implemented".to_string(),
        ))
    }
}

impl TryFrom<Vec<QSOField>> for QSO {
    type Error = AdifError;

    fn try_from(value: Vec<QSOField>) -> Result<Self, Self::Error> {
        Ok(Self { qso: value })
    }
}

#[cfg(test)]
mod tests {
    use crate::adif::AdifItem;
    use crate::data::AdifData;
    use crate::field::Field;
    use crate::fields::data::DataValue;
    use crate::fields::qso::QSOFieldName;
    use crate::qso::{QSOField, QSO};

    #[test]
    fn test_qso_field_serialize() {
        let field = QSOField::new(QSOFieldName::CALL, DataValue::String("IS0GVH".to_string()));
        assert_eq!(field.serialize(), "<CALL:6>IS0GVH");
    }

    #[test]
    fn test_qso_field_deserialize_valid() {
        let input = "<CALL:6>IS0GVH";
        let expected = QSOField::new(QSOFieldName::CALL, DataValue::String("IS0GVH".to_string()));
        let actual = QSOField::deserialize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_qso_field_deserialize_invalid() {
        assert_eq!(QSOField::deserialize("").is_err(), true);
        assert_eq!(QSOField::deserialize("<>").is_err(), true);
        assert_eq!(QSOField::deserialize("<CALL>IS0GVH").is_err(), true);
        assert_eq!(QSOField::deserialize("<CALL:>IS0GVH").is_err(), true);
        assert_eq!(QSOField::deserialize("<INVALID:6>IS0GVH").is_err(), true);
    }

    #[test]
    fn test_qso_add_end_if_missing_add() {
        let now = chrono::Utc::now();
        let input = QSO::try_from(vec![
            QSOField::new(QSOFieldName::CALL, DataValue::String("IS0GVH".to_string())),
            QSOField::new(
                QSOFieldName::QSO_DATE,
                DataValue::Date(now.clone().date_naive()),
            ),
        ])
        .unwrap();
        let expected = QSO::try_from(vec![
            QSOField::new(QSOFieldName::CALL, DataValue::String("IS0GVH".to_string())),
            QSOField::new(
                QSOFieldName::QSO_DATE,
                DataValue::Date(now.clone().date_naive()),
            ),
            QSOField::end(),
        ])
        .unwrap();
        let actual = input.add_end_if_missing();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_qso_add_end_if_missing_already_present() {
        let now = chrono::Utc::now();
        let input = QSO::try_from(vec![
            QSOField::new(QSOFieldName::CALL, DataValue::String("IS0GVH".to_string())),
            QSOField::new(
                QSOFieldName::QSO_DATE,
                DataValue::Date(now.clone().date_naive()),
            ),
            QSOField::end(),
        ])
        .unwrap();
        let expected = input.clone();
        let actual = input.add_end_if_missing();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_qso_serialize() {
        let now = chrono::Utc::now();
        let input = QSO::try_from(vec![
            QSOField::new(
                QSOFieldName::QSO_DATE,
                DataValue::Date(now.clone().date_naive()),
            ),
            QSOField::new(QSOFieldName::CALL, DataValue::String("IS0GVH".to_string())),
            QSOField::end(),
        ])
        .unwrap();
        let expected = format!("<QSO_DATE:8>{}<CALL:6>IS0GVH<EOR>", now.format("%Y%m%d"));
        let actual = input.serialize();
        assert_eq!(expected, actual);
    }
}
