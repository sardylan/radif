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
pub enum Band {
    #[adif("2190M")]
    Band2190m,
    #[adif("630M")]
    Band630m,
    #[adif("560M")]
    Band560m,
    #[adif("160M")]
    Band160m,
    #[adif("80M")]
    Band80m,
    #[adif("60M")]
    Band60m,
    #[adif("40M")]
    Band40m,
    #[adif("30M")]
    Band30m,
    #[adif("20M")]
    Band20m,
    #[adif("17M")]
    Band17m,
    #[adif("15M")]
    Band15m,
    #[adif("12M")]
    Band12m,
    #[adif("10M")]
    Band10m,
    #[adif("8M")]
    Band8m,
    #[adif("6M")]
    Band6m,
    #[adif("5M")]
    Band5m,
    #[adif("4M")]
    Band4m,
    #[adif("2M")]
    Band2m,
    #[adif("1.25M")]
    Band1_25m,
    #[adif("70CM")]
    Band70cm,
    #[adif("33CM")]
    Band33cm,
    #[adif("23CM")]
    Band23cm,
    #[adif("13CM")]
    Band13cm,
    #[adif("9CM")]
    Band9cm,
    #[adif("6CM")]
    Band6cm,
    #[adif("3CM")]
    Band3cm,
    #[adif("1.25CM")]
    Band1_25cm,
    #[adif("6MM")]
    Band6mm,
    #[adif("4MM")]
    Band4mm,
    #[adif("2.5MM")]
    Band2_5mm,
    #[adif("2MM")]
    Band2mm,
    #[adif("1MM")]
    Band1mm,
    #[adif("SUBMM")]
    Bandsubmm,
}
