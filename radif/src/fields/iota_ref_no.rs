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

use crate::data::AdifData;
use crate::enumerations::continent::Continent;
use crate::error::AdifError;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IotaRefNo {
    pub continent: Continent,
    pub number: u16,
}

impl AdifData for IotaRefNo {
    fn serialize(&self) -> String {
        format!("{}-{:03}", self.continent, self.number)
    }

    fn deserialize(value: &str) -> crate::result::Result<Self>
    where
        Self: Sized,
    {
        let items: Vec<&str> = value.split('-').collect();
        if items.len() != 2 {
            return Err(AdifError::DeserializeError(format!(
                "Invalid IOTA reference number format: '{}'. Expected format: 'Continent-Number'",
                value
            )));
        }
        if items[1].len() != 3 {
            return Err(AdifError::DeserializeError(format!(
                "Invalid number format in IOTA reference number: '{}'. Expected 3 digits.",
                items[1]
            )));
        }
        Ok(IotaRefNo {
            continent: Continent::deserialize(items[0])?,
            number: items[1].parse::<u16>().map_err(|_| {
                AdifError::DeserializeError(format!(
                    "Invalid number in IOTA reference number: '{}'",
                    items[1]
                ))
            })?,
        })
    }
}

impl Display for IotaRefNo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::AdifData;
    use crate::enumerations::continent::Continent;
    use crate::fields::iota_ref_no::IotaRefNo;

    #[test]
    fn test_serialize_valid() {
        let input = "EU-024";
        let expected = IotaRefNo {
            continent: Continent::Europe,
            number: 24,
        };
        let actual = IotaRefNo::deserialize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_serialize_invalid_no_dash() {
        assert_eq!(IotaRefNo::deserialize("EU024").is_err(), true);
    }

    #[test]
    fn test_serialize_invalid_no_trailing_zeroes() {
        assert_eq!(IotaRefNo::deserialize("EU-24").is_err(), true);
    }

    #[test]
    fn test_serialize_invalid_continent() {
        assert_eq!(IotaRefNo::deserialize("AA-024").is_err(), true);
    }

    #[test]
    fn test_serialize_invalid_too_much_digits() {
        assert_eq!(IotaRefNo::deserialize("AA-0024").is_err(), true);
    }
}
