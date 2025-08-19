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

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AdifError {
    SerializeError(String),
    DeserializeError(String),
    GenericError(String),
}

impl Display for AdifError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdifError::SerializeError(message) => write!(f, "SerializeError: {}", message),
            AdifError::DeserializeError(message) => write!(f, "DeserializeError: {}", message),
            AdifError::GenericError(message) => write!(f, "GenericError: {}", message),
        }
    }
}

impl Error for AdifError {}

// impl From<> for AdifError {}