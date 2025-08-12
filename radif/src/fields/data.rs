use crate::data::AdifData;
use crate::enumerations::award::Award;
use crate::enumerations::credit::Credit;
use crate::enumerations::{Enumeration, EnumerationType};
use crate::error::AdifError::DeserializeError;
use crate::fields::iota_ref_no::IotaRefNo;
use crate::fields::location::Location;
use crate::fields::sponsored_award::SponsoredAward;
use crate::result;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    AwardList,
    CreditList,
    SponsoredAwardList,
    Boolean,
    Digit,
    Integer,
    Number,
    PositiveInteger,
    Character,
    IntlCharacter,
    Date,
    Time,
    IotaRefNo,
    String,
    IntlString,
    MultilineString,
    IntlMultilineString,
    Enumeration(EnumerationType),
    GridSquare,
    GridSquareExt,
    GridSquareList,
    Location,
    PotaRef,
    PotaRefList,
    SecondarySubdivisionList,
    SecondaryAdministrativeSubdivisionListAlt,
    SotaRef,
    WwffRef,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    AwardList(Vec<Award>),
    CreditList(Vec<Credit>),
    SponsoredAwardList(Vec<SponsoredAward>),
    Boolean(bool),
    Digit(u8),
    Integer(i64),
    Number(f64),
    PositiveInteger(u64),
    Character(char),
    IntlCharacter(char),
    Date(chrono::NaiveDate),
    Time(chrono::NaiveTime),
    IotaRefNo(IotaRefNo),
    String(String),
    IntlString(String),
    MultilineString(String),
    IntlMultilineString(String),
    Enumeration(Enumeration),
    GridSquare(String),
    GridSquareExt(String),
    GridSquareList(Vec<String>),
    Location(Location),
    PotaRef(String),
    PotaRefList(Vec<String>),
    SecondarySubdivisionList(Vec<String>),
    SecondaryAdministrativeSubdivisionListAlt(Vec<String>),
    SotaRef(String),
    WwffRef(String),
}

impl AdifData for DataValue {
    fn serialize(&self) -> String {
        match self {
            DataValue::AwardList(v) => format!("{}", join_vec(v, ",")),
            DataValue::CreditList(v) => format!("{}", join_vec(v, ",")),
            DataValue::SponsoredAwardList(v) => format!("{}", join_vec(v, ",")),
            DataValue::Boolean(v) => format!("{}", if *v { "Y" } else { "N" }),
            DataValue::Digit(v) => format!("{}", *v),
            DataValue::Integer(v) => format!("{}", *v),
            DataValue::Number(v) => format!("{}", *v),
            DataValue::PositiveInteger(v) => format!("{}", *v),
            DataValue::Character(v) => format!("{}", *v),
            DataValue::IntlCharacter(v) => format!("{}", *v),
            DataValue::Date(v) => format!("{}", v.format("%Y%m%d")),
            DataValue::Time(v) => format!("{}", v.format("%H%M%S")),
            DataValue::IotaRefNo(v) => format!("{}", *v),
            DataValue::String(v) => format!("{}", *v),
            DataValue::IntlString(v) => format!("{}", *v),
            DataValue::MultilineString(v) => format!("{}", *v),
            DataValue::IntlMultilineString(v) => format!("{}", *v),
            DataValue::Enumeration(v) => format!("{}", *v),
            DataValue::GridSquare(v) => format!("{}", *v),
            DataValue::GridSquareExt(v) => format!("{}", *v),
            DataValue::GridSquareList(v) => format!("{}", join_vec(v, ",")),
            DataValue::Location(v) => format!("{}", *v),
            DataValue::PotaRef(v) => format!("{}", *v),
            DataValue::PotaRefList(v) => format!("{}", join_vec(v, ",")),
            DataValue::SecondarySubdivisionList(v) => format!("{}", join_vec(v, ",")),
            DataValue::SecondaryAdministrativeSubdivisionListAlt(v) => {
                format!("{}", join_vec(v, ","))
            }
            DataValue::SotaRef(v) => format!("{}", *v),
            DataValue::WwffRef(v) => format!("{}", *v),
        }
    }

    fn deserialize(value: &str) -> result::Result<Self>
    where
        Self: Sized,
    {
        Err(DeserializeError(
            "DataValue does not support direct deserialization from string. Use str_to_enum()."
                .to_string(),
        ))
    }
}

impl Display for DataValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}

impl DataValue {
    pub fn to_char(&self) -> Option<char> {
        match self {
            DataValue::Boolean(_) => Some('B'),
            DataValue::Number(_) => Some('N'),
            DataValue::Date(_) => Some('D'),
            DataValue::Time(_) => Some('T'),
            DataValue::String(_) => Some('S'),
            DataValue::IntlString(_) => Some('I'),
            DataValue::MultilineString(_) => Some('M'),
            DataValue::IntlMultilineString(_) => Some('G'),
            DataValue::Enumeration(_) => Some('E'),
            DataValue::Location(_) => Some('L'),
            _ => None,
        }
    }

    pub fn str_to_enum(data_type: DataType, value: &str) -> result::Result<Self> {
        match data_type {
            DataType::AwardList => Ok(DataValue::AwardList(split_to_vec::<Award>(value)?)),
            DataType::CreditList => Ok(DataValue::CreditList(split_to_vec::<Credit>(value)?)),
            DataType::SponsoredAwardList => Ok(DataValue::SponsoredAwardList(split_to_vec::<
                SponsoredAward,
            >(value)?)),
            DataType::Boolean => Ok(DataValue::Boolean(value == "Y")),
            DataType::Digit => value
                .parse::<u8>()
                .map(DataValue::Digit)
                .map_err(|e| DeserializeError(e.to_string())),
            DataType::Integer => value
                .parse::<i64>()
                .map(DataValue::Integer)
                .map_err(|e| DeserializeError(e.to_string())),
            DataType::Number => value
                .parse::<f64>()
                .map(DataValue::Number)
                .map_err(|e| DeserializeError(e.to_string())),
            DataType::PositiveInteger => value
                .parse::<u64>()
                .map(DataValue::PositiveInteger)
                .map_err(|e| DeserializeError(e.to_string())),
            DataType::Character => value
                .chars()
                .next()
                .map(DataValue::Character)
                .ok_or(DeserializeError(format!("Invalid character '{}'", value))),
            DataType::IntlCharacter => value
                .chars()
                .next()
                .map(DataValue::IntlCharacter)
                .ok_or(DeserializeError(format!("Invalid character '{}'", value))),
            DataType::Date => chrono::NaiveDate::parse_from_str(value, "%Y%m%d")
                .map(DataValue::Date)
                .map_err(|e| DeserializeError(e.to_string())),
            DataType::Time => chrono::NaiveTime::parse_from_str(value, "%H%M%S")
                .map(DataValue::Time)
                .map_err(|e| DeserializeError(e.to_string())),
            DataType::IotaRefNo => IotaRefNo::deserialize(value)
                .map(DataValue::IotaRefNo)
                .map_err(|e| DeserializeError(e.to_string())),
            DataType::String => Ok(DataValue::String(value.to_string())),
            DataType::IntlString => Ok(DataValue::IntlString(value.to_string())),
            DataType::MultilineString => Ok(DataValue::MultilineString(value.to_string())),
            DataType::IntlMultilineString => Ok(DataValue::IntlMultilineString(value.to_string())),
            DataType::Enumeration(e) => {
                Ok(Enumeration::str_to_enum(e, value).map(DataValue::Enumeration)?)
            }
            DataType::GridSquare => Ok(DataValue::GridSquare(value.to_string())),
            DataType::GridSquareExt => Ok(DataValue::GridSquareExt(value.to_string())),
            DataType::GridSquareList => Ok(DataValue::GridSquareList(
                value.split(',').map(String::from).collect(),
            )),
            DataType::Location => Ok(DataValue::Location(
                Location::deserialize(value).map_err(|e| DeserializeError(e.to_string()))?,
            )),
            DataType::PotaRef => Ok(DataValue::PotaRef(value.to_string())),
            DataType::PotaRefList => Ok(DataValue::PotaRefList(
                value.split(',').map(String::from).collect(),
            )),
            DataType::SecondarySubdivisionList => Ok(DataValue::SecondarySubdivisionList(
                value.split(',').map(String::from).collect(),
            )),
            DataType::SecondaryAdministrativeSubdivisionListAlt => {
                Ok(DataValue::SecondaryAdministrativeSubdivisionListAlt(
                    value.split(',').map(String::from).collect(),
                ))
            }
            DataType::SotaRef => Ok(DataValue::SotaRef(value.to_string())),
            DataType::WwffRef => Ok(DataValue::WwffRef(value.to_string())),
        }
    }
}

fn join_vec<T: ToString>(vec: &Vec<T>, s: &str) -> String {
    vec.iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
        .join(s)
}

fn split_to_vec<T: AdifData>(value: &str) -> result::Result<Vec<T>> {
    value
        .split(',')
        .map(|s| T::deserialize(s))
        .collect::<result::Result<Vec<T>>>()
}
