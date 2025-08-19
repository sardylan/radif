use crate::data::AdifData;
use crate::field::Field;
use crate::header::Header;
use crate::qso::QSO;
use std::fmt::{Debug, Display, Formatter};

pub trait AdifItem: AdifData + Debug + Clone + PartialEq + Default {
    type Field: Field;

    fn add_end_if_missing(&self) -> Self;

    fn add_field(&self, field: &Self::Field) -> Self;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Adif {
    pub header: Header,
    pub qso: Vec<QSO>,
}

impl Default for Adif {
    fn default() -> Self {
        Self {
            header: Header::default(),
            qso: vec![],
        }
    }
}

impl AdifData for Adif {
    fn serialize(&self) -> String {
        std::iter::once(self.header.serialize())
            .chain(self.qso.iter().map(|qs| qs.serialize()))
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn deserialize(value: &str) -> crate::result::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Display for Adif {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Headers: {} - QSO: {}",
            self.header.len(),
            self.qso.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::adif::Adif;
    use crate::data::AdifData;
    use crate::field::Field;
    use crate::fields::data::DataValue;
    use crate::fields::header::HeaderFieldName;
    use crate::fields::qso::QSOFieldName;
    use crate::header::{Header, HeaderField};
    use crate::qso::{QSOField, QSO};

    #[test]
    fn test_adif_serialization_empty() {
        let input = Adif {
            header: Header::try_from(vec![]).unwrap(),
            qso: vec![],
        };
        let expected = "";
        let actual = input.serialize();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_adif_serialization_only_header_single() {
        let input = Adif {
            header: Header::try_from(vec![HeaderField::new(
                HeaderFieldName::PROGRAMID,
                DataValue::String("testtest".to_string()),
            )])
            .unwrap(),
            qso: vec![],
        };
        let expected = "<PROGRAMID:8>testtest";
        let actual = input.serialize();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_adif_serialization_only_header_multi() {
        let input = Adif {
            header: Header::try_from(vec![
                HeaderField::new(
                    HeaderFieldName::PROGRAMID,
                    DataValue::String("testtest".to_string()),
                ),
                HeaderField::new(
                    HeaderFieldName::USERDEF(1),
                    DataValue::String("testing".to_string()),
                ),
            ])
            .unwrap(),
            qso: vec![],
        };
        let expected = "<PROGRAMID:8>testtest<USERDEF1:7>testing";
        let actual = input.serialize();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_adif_serialization_only_one_qso_single() {
        let input = Adif {
            header: Header::try_from(vec![]).unwrap(),
            qso: vec![QSO::try_from(vec![QSOField::new(
                QSOFieldName::CALL,
                DataValue::String("IS0GVH".to_string()),
            )])
            .unwrap()],
        };
        let expected = "<CALL:6>IS0GVH";
        let actual = input.serialize();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_adif_serialization_only_one_qso_multi() {
        let input = Adif {
            header: Header::try_from(vec![]).unwrap(),
            qso: vec![QSO::try_from(vec![
                QSOField::new(QSOFieldName::CALL, DataValue::String("IS0GVH".to_string())),
                QSOField::new(QSOFieldName::RST_SENT, DataValue::String("599".to_string())),
            ])
            .unwrap()],
        };
        let expected = "<CALL:6>IS0GVH<RST_SENT:3>599";
        let actual = input.serialize();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_adif_serialization_only_two_qso_single() {
        let input = Adif {
            header: Header::try_from(vec![]).unwrap(),
            qso: vec![
                QSO::try_from(vec![QSOField::new(
                    QSOFieldName::CALL,
                    DataValue::String("IS0GVH".to_string()),
                )])
                .unwrap(),
                QSO::try_from(vec![QSOField::new(
                    QSOFieldName::CALL,
                    DataValue::String("IS0xx".to_string()),
                )])
                .unwrap(),
            ],
        };
        let expected = "<CALL:6>IS0GVH\n<CALL:5>IS0xx";
        let actual = input.serialize();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_adif_serialization_only_two_qso_multi() {
        let input = Adif {
            header: Header::try_from(vec![]).unwrap(),
            qso: vec![
                QSO::try_from(vec![
                    QSOField::new(QSOFieldName::CALL, DataValue::String("IS0GVH".to_string())),
                    QSOField::new(QSOFieldName::RST_SENT, DataValue::String("599".to_string())),
                ])
                .unwrap(),
                QSO::try_from(vec![
                    QSOField::new(QSOFieldName::CALL, DataValue::String("IS0xx".to_string())),
                    QSOField::new(QSOFieldName::RST_SENT, DataValue::String("59".to_string())),
                ])
                .unwrap(),
            ],
        };
        let expected = "<CALL:6>IS0GVH<RST_SENT:3>599\n<CALL:5>IS0xx<RST_SENT:2>59";
        let actual = input.serialize();
        assert_eq!(actual, expected);
    }
}
