/*
 * radif
 * Copyright (C) 2025 - Luca Cireddu (IS0GVH) <sardylan@gmail.com>
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::adif::AdifItem;
use crate::data::AdifData;
use crate::error::AdifError;
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

impl Default for HeaderField {
    fn default() -> Self {
        HeaderField {
            name: HeaderFieldName::EOH,
            value: DataValue::Null(),
            number: None,
        }
    }
}

impl Field for HeaderField {
    type FN = HeaderFieldName;

    fn get_name(&self) -> &Self::FN {
        &self.name
    }

    fn get_value(&self) -> &DataValue {
        &self.value
    }

    fn get_name_end(&self) -> &Self::FN {
        &HeaderFieldName::EOH
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

    fn end() -> Self {
        Self {
            name: HeaderFieldName::EOH,
            value: DataValue::Null(),
            number: None,
        }
    }
}

impl Display for HeaderField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    header: Vec<HeaderField>,
}

impl Header {
    pub fn len(&self) -> usize {
        self.header.len()
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            header: vec![],
        }
    }
}

impl AdifItem for Header {
    type Field = HeaderField;

    fn add_end_if_missing(&self) -> Self {
        self.header
            .last()
            .map_or_else(|| true, |last| last.name != HeaderFieldName::EOH)
            .then(|| Self {
                header: self
                    .header
                    .iter()
                    .cloned()
                    .chain(std::iter::once(HeaderField::end()))
                    .collect(),
            })
            .unwrap_or_else(|| self.clone())
    }

    fn add_field(&self, field: &Self::Field) -> Self {
        Self {
            header: self
                .header
                .clone()
                .into_iter()
                .chain(std::iter::once(field.clone()))
                .collect(),
        }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

impl AdifData for Header {
    fn serialize(&self) -> String {
        self.header
            .iter()
            .map(HeaderField::serialize)
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

impl TryFrom<Vec<HeaderField>> for Header {
    type Error = AdifError;

    fn try_from(value: Vec<HeaderField>) -> Result<Self, Self::Error> {
        Ok(Self { header: value })
    }
}

#[cfg(test)]
mod tests {
    use crate::adif::AdifItem;
    use crate::data::AdifData;
    use crate::field::Field;
    use crate::fields::data::DataValue;
    use crate::fields::header::HeaderFieldName;
    use crate::header::{Header, HeaderField};

    #[test]
    fn test_header_field_serialize() {
        let field = HeaderField::new(
            HeaderFieldName::PROGRAMID,
            DataValue::String("Test".to_string()),
        );
        assert_eq!(field.serialize(), "<PROGRAMID:4>Test");
    }

    #[test]
    fn test_header_field_deserialize_valid() {
        let input = "<PROGRAMID:4>Test";
        let expected = HeaderField::new(
            HeaderFieldName::PROGRAMID,
            DataValue::String("Test".to_string()),
        );
        let actual = HeaderField::deserialize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_header_field_deserialize_invalid() {
        assert_eq!(HeaderField::deserialize("").is_err(), true);
        assert_eq!(HeaderField::deserialize("<>").is_err(), true);
        assert_eq!(HeaderField::deserialize("<PROGRAMID>Test").is_err(), true);
        assert_eq!(HeaderField::deserialize("<PROGRAMID:>Test").is_err(), true);
        assert_eq!(HeaderField::deserialize("<INVALID:6>Test").is_err(), true);
    }

    #[test]
    fn test_header_add_end_if_missing_add() {
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string();
        let input = Header::try_from(vec![
            HeaderField::new(
                HeaderFieldName::PROGRAMID,
                DataValue::String("Test".to_string()),
            ),
            HeaderField::new(
                HeaderFieldName::CREATED_TIMESTAMP,
                DataValue::String(now.clone()),
            ),
        ])
        .unwrap();
        let expected = Header::try_from(vec![
            HeaderField::new(
                HeaderFieldName::PROGRAMID,
                DataValue::String("Test".to_string()),
            ),
            HeaderField::new(HeaderFieldName::CREATED_TIMESTAMP, DataValue::String(now)),
            HeaderField::end(),
        ])
        .unwrap();
        let actual = input.add_end_if_missing();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_header_add_end_if_missing_already_present() {
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string();
        let input = Header::try_from(vec![
            HeaderField::new(
                HeaderFieldName::PROGRAMID,
                DataValue::String("IS0GVH".to_string()),
            ),
            HeaderField::new(HeaderFieldName::CREATED_TIMESTAMP, DataValue::String(now)),
            HeaderField::end(),
        ])
        .unwrap();
        let expected = input.clone();
        let actual = input.add_end_if_missing();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_header_serialize() {
        let now = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string();
        let input = Header::try_from(vec![
            HeaderField::new(
                HeaderFieldName::PROGRAMID,
                DataValue::String("Test".to_string()),
            ),
            HeaderField::new(
                HeaderFieldName::CREATED_TIMESTAMP,
                DataValue::String(now.clone()),
            ),
            HeaderField::end(),
        ])
        .unwrap();
        let expected = format!(
            "<PROGRAMID:4>Test<CREATED_TIMESTAMP:{}>{}<EOH>",
            now.len(),
            now
        );
        let actual = input.serialize();
        assert_eq!(expected, actual);
    }
}
