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

use crate::error::AdifError;
use crate::error::AdifError::GenericError;
use crate::result;
use futures::{stream, StreamExt, TryFutureExt, TryStreamExt};
use std::fmt::{Display, Formatter};

const SHIFT_0: (f64, f64) = (20.0, 10.0);
const SHIFT_1: (f64, f64) = (2.0, 1.0);
const SHIFT_2: (f64, f64) = ((1.0 / 60.0) * 5.0, (1.0 / 60.0) * 2.5);
const SHIFT_3: (f64, f64) = ((1.0 / 3600.0) * 30.0, (1.0 / 3600.0) * 15.0);
const SHIFT_4: (f64, f64) = ((1.0 / 3600.0) * 1.25, (1.0 / 3600.0) * 0.625);
const SHIFT_5: (f64, f64) = ((1.0 / 3600.0) * 0.125, (1.0 / 3600.0) * 0.0625);

const DELTA_2: (f64, f64) = (SHIFT_0.0 / 2.0, SHIFT_0.1 / 2.0);
const DELTA_4: (f64, f64) = (SHIFT_1.0 / 2.0, SHIFT_1.1 / 2.0);
const DELTA_6: (f64, f64) = (SHIFT_2.0 / 2.0, SHIFT_2.1 / 2.0);
const DELTA_8: (f64, f64) = (SHIFT_3.0 / 2.0, SHIFT_3.1 / 2.0);
const DELTA_10: (f64, f64) = (SHIFT_4.0 / 2.0, SHIFT_4.1 / 2.0);
const DELTA_12: (f64, f64) = (SHIFT_5.0 / 2.0, SHIFT_5.1 / 2.0);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub longitude: f64,
    pub latitude: f64,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.latitude, self.longitude)
    }
}

impl TryFrom<(f64, f64)> for Point {
    type Error = AdifError;

    fn try_from(value: (f64, f64)) -> Result<Self, Self::Error> {
        Ok(Self {
            longitude: value.0,
            latitude: value.1,
        })
    }
}

impl Point {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self {
            longitude,
            latitude,
        }
    }
}

pub async fn locator_to_coordinates(locator: &str) -> result::Result<Point> {
    let delta_center = match locator.len() {
        2 => DELTA_2,
        4 => DELTA_4,
        6 => DELTA_6,
        8 => DELTA_8,
        10 => DELTA_10,
        12 => DELTA_12,
        _ => {
            return Err(GenericError(
                "Locator length must be 2, 4, 6, 8, 10 or 12 characters".to_string(),
            ));
        }
    };

    let characters: Vec<(usize, u8, u8)> = locator
        .to_ascii_uppercase()
        .as_bytes()
        .chunks(2)
        .enumerate()
        .map(|(index, item)| (index, item[0], item[1]))
        .collect();

    stream::iter(characters)
        .then(validate_characters)
        .and_then(convert_characters_to_numbers)
        .and_then(compute_shift)
        .try_fold(delta_center, sum_coordinates)
        .and_then(half_divide)
        .and_then(|x| async move { x.try_into() })
        .await
}

async fn validate_characters(
    (index, long, lat): (usize, u8, u8),
) -> result::Result<(usize, u8, u8)> {
    match index {
        0 => {
            if lat < b'A' || lat > b'R' || long < b'A' || long > b'R' {
                return Err(GenericError("Invalid locator character".to_string()));
            }
        }
        2 | 4 => {
            if lat < b'A' || lat > b'X' || long < b'A' || long > b'X' {
                return Err(GenericError("Invalid locator character".to_string()));
            }
        }
        _ => {
            if lat < b'0' || lat > b'9' || long < b'0' || long > b'9' {
                return Err(GenericError("Invalid locator character".to_string()));
            }
        }
    }
    Ok((index, long, lat))
}

async fn convert_characters_to_numbers(
    (index, long, lat): (usize, u8, u8),
) -> result::Result<(usize, f64, f64)> {
    Ok(if index % 2 == 0 {
        (index, (long - b'A') as f64, (lat - b'A') as f64)
    } else {
        (index, (long - b'0') as f64, (lat - b'0') as f64)
    })
}

async fn compute_shift((index, long, lat): (usize, f64, f64)) -> result::Result<(f64, f64)> {
    let (long_shift, lat_shift) = match index {
        0 => (long * SHIFT_0.0, lat * SHIFT_0.1),
        1 => (long * SHIFT_1.0, lat * SHIFT_1.1),
        2 => (long * SHIFT_2.0, lat * SHIFT_2.1),
        3 => (long * SHIFT_3.0, lat * SHIFT_3.1),
        4 => (long * SHIFT_4.0, lat * SHIFT_4.1),
        5 => (long * SHIFT_5.0, lat * SHIFT_5.1),
        _ => (0.0, 0.0),
    };
    Ok((long_shift, lat_shift))
}

async fn sum_coordinates(
    (long_acc, lat_acc): (f64, f64),
    (long, lat): (f64, f64),
) -> result::Result<(f64, f64)> {
    Ok((long_acc + long, lat_acc + lat))
}

async fn half_divide((long, lat): (f64, f64)) -> result::Result<(f64, f64)> {
    Ok((long - 180.0, lat - 90.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    #[test]
    fn test_valid_two() {
        let result = block_on(locator_to_coordinates("JM")).unwrap();
        assert_eq!(result, Point::new(10.0, 35.0));
    }

    #[test]
    fn test_valid_four() {
        let result = block_on(locator_to_coordinates("JM49")).unwrap();
        assert_eq!(result, Point::new(9.0, 39.5));
    }

    #[test]
    fn test_valid_six() {
        let result = block_on(locator_to_coordinates("JM49SK")).unwrap();
        assert_eq!(result, Point::new(9.541666666666657, 39.43749999999997));
    }

    #[test]
    fn test_valid_eigth() {
        let result = block_on(locator_to_coordinates("JM49SK46")).unwrap();
        assert_eq!(result, Point::new(9.537499999999994, 39.443749999999994));
    }

    #[test]
    fn test_valid_ten() {
        let result = block_on(locator_to_coordinates("JM49SK46XV")).unwrap();
        assert_eq!(result, Point::new(9.541493055555549, 39.445399305555554));
    }

    #[test]
    fn test_valid_twelve() {
        let result = block_on(locator_to_coordinates("JM49SK46XV94")).unwrap();
        assert_eq!(result, Point::new(9.541649305555552, 39.44539062499999));
    }

    #[test]
    fn test_invalid_empty() {
        let result = block_on(locator_to_coordinates(""));
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_locator_length() {
        let result = block_on(locator_to_coordinates("JO1"));
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_locator_characters() {
        let result = block_on(locator_to_coordinates("ZZ00"));
        assert!(result.is_err());
    }
}
