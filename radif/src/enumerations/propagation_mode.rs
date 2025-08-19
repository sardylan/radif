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
use crate::error::AdifError;
use radif_macros::{AdifData, AutoDisplay, AutoTestEnum};

#[derive(Debug, Clone, PartialEq, Eq, AutoDisplay, AdifData, AutoTestEnum)]
pub enum PropagationMode {
    #[adif("AS")]
    AircraftScatter,
    #[adif("AUE")]
    AuroraE,
    #[adif("AUR")]
    Aurora,
    #[adif("BS")]
    BackScatter,
    #[adif("ECH")]
    EchoLink,
    #[adif("EME")]
    EarthMoonEarth,
    #[adif("ES")]
    SporadicE,
    #[adif("F2")]
    F2Reflection,
    #[adif("FAI")]
    FieldAlignedIrregularities,
    #[adif("GWAVE")]
    GroundWave,
    #[adif("INTERNET")]
    InternetAssisted,
    #[adif("ION")]
    Ionoscatter,
    #[adif("IRL")]
    IRLP,
    #[adif("LOS")]
    LineOfSight,
    #[adif("MS")]
    MeteorScatter,
    #[adif("RPT")]
    Repeater,
    #[adif("RS")]
    RainScatter,
    #[adif("SAT")]
    Satellite,
    #[adif("TEP")]
    TransEquatorial,
    #[adif("TR")]
    TroposphericDucting,
}
