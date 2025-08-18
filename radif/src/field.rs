use crate::data::AdifData;
use crate::error::AdifError::DeserializeError;
use crate::fields::data::{DataType, DataValue};

pub trait FieldName: AdifData + PartialEq {
    fn get_data_type(&self) -> DataType;

    fn get_value_type_char(&self) -> Option<String> {
        None
    }
}

pub trait Field: AdifData + PartialEq {
    type FN: FieldName;

    fn get_name(&self) -> &Self::FN;
    fn get_value(&self) -> &DataValue;

    fn get_name_end(&self) -> &Self::FN;

    fn new(name: Self::FN, value: DataValue) -> Self;

    fn end() -> Self;
}

impl<T: Field> AdifData for T {
    fn serialize(&self) -> String {
        let value = self.get_value().serialize();

        if let Some(vt) = self.get_name().get_value_type_char() {
            format!(
                "<{}:{}:{}>{}",
                self.get_name().serialize(),
                vt,
                value.len(),
                value
            )
        } else if self.get_name() == self.get_name_end() {
            format!("<{}>", self.get_name().serialize())
        } else {
            format!("<{}:{}>{}", self.get_name().serialize(), value.len(), value)
        }
    }

    fn deserialize(string: &str) -> crate::result::Result<T>
    where
        Self: Sized,
    {
        if !string.starts_with("<") {
            return Err(DeserializeError(
                "Invalid field format: missing '<' at the start".to_string(),
            ));
        }

        let end_index = string
            .find('>')
            .ok_or_else(|| DeserializeError("Invalid field format: missing '>'".to_string()))?;
        let header = &string[1..end_index];
        let parts: Vec<&str> = header.split(':').collect();
        if parts.len() < 2 {
            return Err(DeserializeError(
                "Invalid field format: expected at least two parts in header".to_string(),
            ));
        }
        let name = T::FN::deserialize(parts[0])?;

        let value_idx = if parts.len() == 2 { 1 } else { 2 };
        let value_length: usize = parts[value_idx].parse().map_err(|_| {
            DeserializeError("Invalid field format: value length is not a number".to_string())
        })?;

        let value_str = &string[end_index + 1..end_index + 1 + value_length];
        if value_str.len() != value_length {
            return Err(DeserializeError(format!(
                "Invalid field format: value length mismatch, expected {}, got {}",
                value_length,
                value_str.len()
            )));
        }

        let value = DataValue::str_to_enum(name.get_data_type(), value_str)?;

        Ok(Self::new(name, value))
    }
}
