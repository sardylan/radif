use crate::data::AdifData;
use crate::error::AdifError::DeserializeError;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LocationDirection {
    North,
    South,
    East,
    West,
}

impl Display for LocationDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LocationDirection::North => write!(f, "N"),
            LocationDirection::South => write!(f, "S"),
            LocationDirection::East => write!(f, "E"),
            LocationDirection::West => write!(f, "W"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub direction: LocationDirection,
    pub value: f64,
}

impl AdifData for Location {
    fn serialize(&self) -> String {
        format!("{}{} {:06.3}", self.direction, self.value as u8, self.value)
    }

    fn deserialize(value: &str) -> crate::result::Result<Self>
    where
        Self: Sized,
    {
        let direction = match value.chars().next() {
            Some('N') => LocationDirection::North,
            Some('S') => LocationDirection::South,
            Some('E') => LocationDirection::East,
            Some('W') => LocationDirection::West,
            _ => return Err(DeserializeError("Invalid location direction".to_string())),
        };

        let items = value[1..].split(' ').collect::<Vec<&str>>();
        if items.len() != 2 {
            return Err(DeserializeError(
                "Location value must have exactly two parts".to_string(),
            ));
        };
        if items[0].len() != 3 || !items[0].chars().all(|c| c.is_digit(10)) {
            return Err(DeserializeError(
                "Location value must have a 3-digit number for degrees".to_string(),
            ));
        };
        if items[1].len() != 6 || !items[1].chars().all(|c| c.is_digit(10) || c == '.') {
            return Err(DeserializeError(
                "Location value must have a 6-digit number for minutes with optional decimal"
                    .to_string(),
            ));
        };

        let degrees: f64 = items[0]
            .parse::<u8>()
            .map_err(|e| DeserializeError(e.to_string()))? as f64;
        if direction == LocationDirection::East || direction == LocationDirection::West {
            if degrees < 0.0 || degrees > 180.0 {
                return Err(DeserializeError(
                    "Degrees for East/West must be between 0 and 180".to_string(),
                ));
            }
        }
        if direction == LocationDirection::North || direction == LocationDirection::South {
            if degrees < 0.0 || degrees > 90.0 {
                return Err(DeserializeError(
                    "Degrees for North/South must be between 0 and 90".to_string(),
                ));
            }
        }

        let minutes: f64 = items[1]
            .parse::<f64>()
            .map_err(|e| DeserializeError(e.to_string()))?
            / 60.0;

        Ok(Self {
            direction,
            value: degrees + minutes,
        })
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::AdifData;
    use crate::fields::location::Location;

    #[test]
    fn test_valid() {
        let input = "N045 12.456";
        let expected = Location {
            direction: super::LocationDirection::North,
            value: 45.207600f64,
        };
        let actual = Location::deserialize(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_invalid_no_space() {
        let input = "N04512.456";
        assert_eq!(Location::deserialize(input).is_err(), true);
    }

    #[test]
    fn test_invalid_no_decimals() {
        let input = "N045 12456";
        assert_eq!(Location::deserialize(input).is_err(), true);
    }

    #[test]
    fn test_invalid_no_leading_zeroes() {
        let input = "N45 12.456";
        assert_eq!(Location::deserialize(input).is_err(), true);
    }

    #[test]
    fn test_invalid_no_direction() {
        let input = "45 12.456";
        assert_eq!(Location::deserialize(input).is_err(), true);
    }

    #[test]
    fn test_invalid_wrong_direction() {
        let input = "Z45 12.456";
        assert_eq!(Location::deserialize(input).is_err(), true);
    }

    #[test]
    fn test_invalid_out_of_range_latitude() {
        let input = "N095 12.456";
        assert_eq!(Location::deserialize(input).is_err(), true);
    }

    #[test]
    fn test_invalid_out_of_range_longitude() {
        let input = "E195 12.456";
        assert_eq!(Location::deserialize(input).is_err(), true);
    }
}
