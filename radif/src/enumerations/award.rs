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
#[allow(non_camel_case_types)]
pub enum Award {
    #[adif("AJA")]
    AJA,
    #[adif("CQDX")]
    CQDX,
    #[adif("CQDXFIELD")]
    CQDXFIELD,
    #[adif("CQWAZ_MIXED")]
    CQWAZ_MIXED,
    #[adif("CQWAZ_CW")]
    CQWAZ_CW,
    #[adif("CQWAZ_PHONE")]
    CQWAZ_PHONE,
    #[adif("CQWAZ_RTTY")]
    CQWAZ_RTTY,
    #[adif("CQWAZ_160M")]
    CQWAZ_160m,
    #[adif("CQWPX")]
    CQWPX,
    #[adif("DARC_DOK")]
    DARC_DOK,
    #[adif("DXCC")]
    DXCC,
    #[adif("DXCC_MIXED")]
    DXCC_MIXED,
    #[adif("DXCC_CW")]
    DXCC_CW,
    #[adif("DXCC_PHONE")]
    DXCC_PHONE,
    #[adif("DXCC_RTTY")]
    DXCC_RTTY,
    #[adif("IOTA")]
    IOTA,
    #[adif("JCC")]
    JCC,
    #[adif("JCG")]
    JCG,
    #[adif("MARATHON")]
    MARATHON,
    #[adif("RDA")]
    RDA,
    #[adif("WAB")]
    WAB,
    #[adif("WAC")]
    WAC,
    #[adif("WAE")]
    WAE,
    #[adif("WAIP")]
    WAIP,
    #[adif("WAJA")]
    WAJA,
    #[adif("WAS")]
    WAS,
    #[adif("WAZ")]
    WAZ,
    #[adif("USACA")]
    USACA,
    #[adif("VUCC")]
    VUCC,
}
