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

use crate::enumerations::ant_path::AntPath;
use crate::enumerations::arrl_section::ArrlSection;
use crate::enumerations::award::Award;
use crate::enumerations::band::Band;
use crate::enumerations::contest_id::ContestId;
use crate::enumerations::continent::Continent;
use crate::enumerations::credit::Credit;
use crate::enumerations::dxcc_enity_code::DxccEntityCode;
use crate::enumerations::mode::Mode;
use crate::enumerations::morse_key_type::MorseKeyType;
use crate::enumerations::propagation_mode::PropagationMode;
use crate::enumerations::qsl_medium::QslMedium;
use crate::enumerations::qsl_rcvd::QslRcvd;
use crate::enumerations::qsl_sent::QslSent;
use crate::enumerations::qsl_via::QslVia;
use crate::enumerations::qso_complete::QsoComplete;
use crate::enumerations::qso_upload_status::QsoUploadStatus;
use crate::enumerations::region::Region;
use crate::enumerations::submode::SubMode;
use crate::result;
use std::fmt::{Display, Formatter};
use crate::data::AdifData;

pub mod ant_path;
pub mod arrl_section;
pub mod award;
pub mod band;
pub mod contest_id;
pub mod continent;
pub mod credit;
pub mod dxcc_enity_code;
pub mod mode;
pub mod morse_key_type;
pub mod propagation_mode;
pub mod qsl_medium;
pub mod qsl_rcvd;
pub mod qsl_sent;
pub mod qsl_via;
pub mod qso_complete;
pub mod qso_upload_status;
pub mod region;
pub mod submode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumerationType {
    AntPath,
    ArrlSection,
    Award,
    Band,
    ContestId,
    Continent,
    Credit,
    DxccEntityCode,
    Mode,
    MorseKeyType,
    PropagationMode,
    QslMedium,
    QslRcvd,
    QslSent,
    QslVia,
    QsoComplete,
    QsoUploadStatus,
    Region,
    SubMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Enumeration {
    AntPath(AntPath),
    ArrlSection(ArrlSection),
    Award(Award),
    Band(Band),
    ContestId(ContestId),
    Continent(Continent),
    Credit(Credit),
    DxccEntityCode(DxccEntityCode),
    Mode(Mode),
    MorseKeyType(MorseKeyType),
    PropagationMode(PropagationMode),
    QslMedium(QslMedium),
    QslRcvd(QslRcvd),
    QslSent(QslSent),
    QslVia(QslVia),
    QsoComplete(QsoComplete),
    QsoUploadStatus(QsoUploadStatus),
    Region(Region),
    SubMode(SubMode),
}

impl Enumeration {
    pub fn str_to_enum(enum_type: EnumerationType, value: &str) -> result::Result<Self> {
        match enum_type {
            EnumerationType::AntPath => AntPath::deserialize(value).map(Enumeration::AntPath),
            EnumerationType::ArrlSection => {
                ArrlSection::deserialize(value).map(Enumeration::ArrlSection)
            }
            EnumerationType::Award => Award::deserialize(value).map(Enumeration::Award),
            EnumerationType::Band => Band::deserialize(value).map(Enumeration::Band),
            EnumerationType::ContestId => ContestId::deserialize(value).map(Enumeration::ContestId),
            EnumerationType::Continent => Continent::deserialize(value).map(Enumeration::Continent),
            EnumerationType::Credit => Credit::deserialize(value).map(Enumeration::Credit),
            EnumerationType::DxccEntityCode => {
                DxccEntityCode::deserialize(value).map(Enumeration::DxccEntityCode)
            }
            EnumerationType::Mode => Mode::deserialize(value).map(Enumeration::Mode),
            EnumerationType::MorseKeyType => {
                MorseKeyType::deserialize(value).map(Enumeration::MorseKeyType)
            }
            EnumerationType::PropagationMode => {
                PropagationMode::deserialize(value).map(Enumeration::PropagationMode)
            }
            EnumerationType::QslMedium => QslMedium::deserialize(value).map(Enumeration::QslMedium),
            EnumerationType::QslRcvd => QslRcvd::deserialize(value).map(Enumeration::QslRcvd),
            EnumerationType::QslSent => QslSent::deserialize(value).map(Enumeration::QslSent),
            EnumerationType::QslVia => QslVia::deserialize(value).map(Enumeration::QslVia),
            EnumerationType::QsoComplete => {
                QsoComplete::deserialize(value).map(Enumeration::QsoComplete)
            }
            EnumerationType::QsoUploadStatus => {
                QsoUploadStatus::deserialize(value).map(Enumeration::QsoUploadStatus)
            }
            EnumerationType::Region => Region::deserialize(value).map(Enumeration::Region),
            EnumerationType::SubMode => SubMode::deserialize(value).map(Enumeration::SubMode),
        }
    }
}

impl Display for Enumeration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Enumeration::AntPath(e) => write!(f, "{}", e),
            Enumeration::ArrlSection(e) => write!(f, "{}", e),
            Enumeration::Award(e) => write!(f, "{}", e),
            Enumeration::Band(e) => write!(f, "{}", e),
            Enumeration::ContestId(e) => write!(f, "{}", e),
            Enumeration::Continent(e) => write!(f, "{}", e),
            Enumeration::Credit(e) => write!(f, "{}", e),
            Enumeration::DxccEntityCode(e) => write!(f, "{}", e),
            Enumeration::Mode(e) => write!(f, "{}", e),
            Enumeration::MorseKeyType(e) => write!(f, "{}", e),
            Enumeration::PropagationMode(e) => write!(f, "{}", e),
            Enumeration::QslMedium(e) => write!(f, "{}", e),
            Enumeration::QslRcvd(e) => write!(f, "{}", e),
            Enumeration::QslSent(e) => write!(f, "{}", e),
            Enumeration::QslVia(e) => write!(f, "{}", e),
            Enumeration::QsoComplete(e) => write!(f, "{}", e),
            Enumeration::QsoUploadStatus(e) => write!(f, "{}", e),
            Enumeration::Region(e) => write!(f, "{}", e),
            Enumeration::SubMode(e) => write!(f, "{}", e),
        }
    }
}
