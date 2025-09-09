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

use crate::adif::{Adif, AdifItem};
use crate::data::AdifData;
use crate::error::AdifError::DeserializeError;
use crate::field::Field;
use crate::header::HeaderField;
use crate::qso::{QSOField, QSO};
use crate::result;
use futures::{AsyncRead, AsyncReadExt};

#[derive(Debug, PartialEq)]
enum FieldState {
    LookingForBeginning,
    InTag,
    InValue,
}

#[derive(Debug, PartialEq)]
enum AdifState {
    InHeader,
    InQso,
}

#[derive(Debug)]
struct State {
    field_state: FieldState,
    adif_state: AdifState,
    buffer: String,
    counter: usize,
    qso: QSO,
    adif: Adif,
}

impl Default for State {
    fn default() -> Self {
        State {
            field_state: FieldState::LookingForBeginning,
            adif_state: AdifState::InHeader,
            buffer: String::with_capacity(2048), // Pre-allocate buffer
            counter: 0,
            qso: QSO::default(),
            adif: Adif::default(),
        }
    }
}

pub async fn parse<R>(mut reader: R) -> result::Result<Adif>
where
    R: AsyncRead + Unpin,
{
    let mut state = State::default();
    let mut buffer = [0u8; 4096]; // Read in chunks for better performance

    loop {
        match reader.read(&mut buffer).await {
            Ok(0) => break, // EOF
            Ok(n) => {
                for &byte in &buffer[..n] {
                    let c = byte as char;
                    parse_adif_char(&mut state, c)?;
                }
            }
            Err(e) => return Err(DeserializeError(e.to_string())),
        }
    }

    Ok(state.adif)
}

#[cfg(feature = "tokio")]
pub async fn parse_tokio<R>(mut reader: R) -> result::Result<Adif>
where
    R: tokio::io::AsyncRead + Unpin,
{
    use tokio::io::AsyncReadExt;

    let mut state = State::default();
    let mut buffer = [0u8; 4096]; // Read in chunks for better performance

    loop {
        match reader.read(&mut buffer).await {
            Ok(0) => break, // EOF
            Ok(n) => {
                for &byte in &buffer[..n] {
                    let c = byte as char;
                    parse_adif_char(&mut state, c)?;
                }
            }
            Err(e) => return Err(DeserializeError(e.to_string())),
        }
    }

    Ok(state.adif)
}

fn parse_adif_char(state: &mut State, c: char) -> result::Result<()> {
    match state.field_state {
        FieldState::LookingForBeginning => {
            if c == '<' {
                state.field_state = FieldState::InTag;
                state.buffer.clear();
                state.buffer.push('<');
                state.counter = 0;
            }
        }
        FieldState::InTag => match c {
            '>' => {
                state.buffer.push('>');

                match state.adif_state {
                    AdifState::InHeader => {
                        if state.buffer.to_uppercase() == HeaderField::end().serialize() {
                            state.field_state = FieldState::LookingForBeginning;
                            state.adif_state = AdifState::InQso;
                            state.adif.header = state.adif.header.add_field(&HeaderField::end());
                            state.buffer.clear();
                            return Ok(());
                        }
                    }
                    AdifState::InQso => {
                        if state.buffer == QSOField::end().serialize() {
                            state.field_state = FieldState::LookingForBeginning;
                            state.qso = state.qso.add_field(&QSOField::end());
                            state.adif.qso.push(std::mem::take(&mut state.qso));
                            state.buffer.clear();
                            return Ok(());
                        }
                    }
                }

                if let Some(colon_pos) = state.buffer.rfind(':') {
                    let length_str = &state.buffer[colon_pos + 1..state.buffer.len() - 1];
                    state.counter = length_str
                        .parse::<usize>()
                        .map_err(|e| DeserializeError(format!("Invalid length: {}", e)))?;
                    state.field_state = FieldState::InValue;
                } else {
                    return Err(DeserializeError("Missing length in tag".to_string()));
                }
            }
            _ => {
                state.buffer.push(c);
            }
        },
        FieldState::InValue => {
            state.buffer.push(c);
            state.counter -= 1;

            if state.counter == 0 {
                match state.adif_state {
                    AdifState::InHeader => {
                        let header_field = HeaderField::deserialize(&state.buffer)?;
                        if header_field.is_end() {
                            state.adif_state = AdifState::InQso;
                        }
                        state.adif.header = state.adif.header.add_field(&header_field);
                    }
                    AdifState::InQso => {
                        let qso_field = QSOField::deserialize(&state.buffer)?;
                        if qso_field.is_end() {
                            state.qso = state.qso.add_field(&qso_field);
                            state.adif.qso.push(std::mem::take(&mut state.qso));
                        } else {
                            state.qso = state.qso.add_field(&qso_field);
                        }
                    }
                }

                state.field_state = FieldState::LookingForBeginning;
                state.buffer.clear();
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::data::DataValue;
    use crate::fields::header::HeaderFieldName;
    use crate::fields::qso::QSOFieldName;
    use crate::header::Header;

    #[test]
    fn test_parse_adif_char_default() {
        let mut state = State::default();
        parse_adif_char(&mut state, ' ').unwrap();
        assert_eq!(state.field_state, FieldState::LookingForBeginning);

        let mut state = State::default();
        parse_adif_char(&mut state, '>').unwrap();
        assert_eq!(state.field_state, FieldState::LookingForBeginning);

        let mut state = State::default();
        parse_adif_char(&mut state, ':').unwrap();
        assert_eq!(state.field_state, FieldState::LookingForBeginning);
    }

    #[test]
    fn test_parse_adif_char_header_first_char() {
        let mut state = State::default();
        parse_adif_char(&mut state, '<').unwrap();
        assert_eq!(state.field_state, FieldState::InTag);
        assert_eq!(state.buffer, "<");
    }

    #[test]
    fn test_parse_adif_char_header_second_char() {
        let mut state = State {
            field_state: FieldState::InTag,
            buffer: "<".to_string(),
            ..State::default()
        };
        parse_adif_char(&mut state, 'A').unwrap();
        assert_eq!(state.field_state, FieldState::InTag);
        assert_eq!(state.buffer, "<A");
    }

    #[test]
    fn test_parse_adif_char_header_separator() {
        let mut state = State {
            field_state: FieldState::InTag,
            buffer: "<PROGRAMID".to_string(),
            ..State::default()
        };
        parse_adif_char(&mut state, ':').unwrap();
        assert_eq!(state.field_state, FieldState::InTag);
        assert_eq!(state.buffer, "<PROGRAMID:");
    }

    #[test]
    fn test_parse_adif_char_header_value_begin() {
        let mut state = State {
            field_state: FieldState::InTag,
            buffer: "<PROGRAMID:4".to_string(),
            ..State::default()
        };
        parse_adif_char(&mut state, '>').unwrap();
        assert_eq!(state.field_state, FieldState::InValue);
        assert_eq!(state.counter, 4);
        assert_eq!(state.buffer, "<PROGRAMID:4>");
    }

    #[test]
    fn test_parse_adif_char_header_value_first_char() {
        let mut state = State {
            field_state: FieldState::InValue,
            counter: 4,
            buffer: "<PROGRAMID:4>".to_string(),
            ..State::default()
        };
        parse_adif_char(&mut state, 't').unwrap();
        assert_eq!(state.field_state, FieldState::InValue);
        assert_eq!(state.counter, 3);
        assert_eq!(state.buffer, "<PROGRAMID:4>t");
    }

    #[test]
    fn test_parse_adif_char_header_value_start_tag_char() {
        let mut state = State {
            field_state: FieldState::InValue,
            counter: 4,
            buffer: "<PROGRAMID:4>".to_string(),
            ..State::default()
        };
        parse_adif_char(&mut state, '<').unwrap();
        assert_eq!(state.field_state, FieldState::InValue);
        assert_eq!(state.counter, 3);
        assert_eq!(state.buffer, "<PROGRAMID:4><");
    }

    #[test]
    fn test_parse_adif_char_header_value_last_char() {
        let mut state = State {
            field_state: FieldState::InValue,
            counter: 1,
            buffer: "<PROGRAMID:4>tes".to_string(),
            ..State::default()
        };
        parse_adif_char(&mut state, 't').unwrap();
        assert_eq!(state.field_state, FieldState::LookingForBeginning);
        assert_eq!(state.counter, 0);
        assert_eq!(state.buffer, "");
        assert_eq!(state.adif.header.len(), 1);
    }

    #[test]
    fn test_parse_adif_char_header_eoh_last_char() {
        let mut state = State {
            field_state: FieldState::InTag,
            counter: 0,
            buffer: "<EOH".to_string(),
            adif: Adif {
                header: Header::try_from(vec![HeaderField::new(
                    HeaderFieldName::PROGRAMID,
                    DataValue::String("test".to_string()),
                )])
                .unwrap(),
                ..Adif::default()
            },
            ..State::default()
        };
        parse_adif_char(&mut state, '>').unwrap();
        assert_eq!(state.field_state, FieldState::LookingForBeginning);
        assert_eq!(state.adif_state, AdifState::InQso);
        assert_eq!(state.buffer, "");
        assert_eq!(state.adif.header.len(), 2);
    }

    #[test]
    fn test_parse_adif_char_qso_first_char() {
        let mut state = State {
            field_state: FieldState::LookingForBeginning,
            adif_state: AdifState::InQso,
            counter: 0,
            buffer: "".to_string(),
            adif: Adif {
                header: Header::try_from(vec![
                    HeaderField::new(
                        HeaderFieldName::PROGRAMID,
                        DataValue::String("test".to_string()),
                    ),
                    HeaderField::new(HeaderFieldName::EOH, DataValue::Null()),
                ])
                .unwrap(),
                ..Adif::default()
            },
            ..State::default()
        };
        parse_adif_char(&mut state, '<').unwrap();
        assert_eq!(state.field_state, FieldState::InTag);
        assert_eq!(state.adif_state, AdifState::InQso);
        assert_eq!(state.buffer, "<");
    }

    #[test]
    fn test_parse_adif_char_qso_second_char() {
        let mut state = State {
            field_state: FieldState::InTag,
            adif_state: AdifState::InQso,
            counter: 0,
            buffer: "<CALL".to_string(),
            adif: Adif {
                header: Header::try_from(vec![
                    HeaderField::new(
                        HeaderFieldName::PROGRAMID,
                        DataValue::String("test".to_string()),
                    ),
                    HeaderField::new(HeaderFieldName::EOH, DataValue::Null()),
                ])
                .unwrap(),
                ..Adif::default()
            },
            ..State::default()
        };
        parse_adif_char(&mut state, ':').unwrap();
        assert_eq!(state.field_state, FieldState::InTag);
        assert_eq!(state.adif_state, AdifState::InQso);
        assert_eq!(state.buffer, "<CALL:");
    }

    #[test]
    fn test_parse_adif_char_qso_last_tag_char() {
        let mut state = State {
            field_state: FieldState::InTag,
            adif_state: AdifState::InQso,
            counter: 0,
            buffer: "<CALL:6".to_string(),
            adif: Adif {
                header: Header::try_from(vec![
                    HeaderField::new(
                        HeaderFieldName::PROGRAMID,
                        DataValue::String("test".to_string()),
                    ),
                    HeaderField::new(HeaderFieldName::EOH, DataValue::Null()),
                ])
                .unwrap(),
                ..Adif::default()
            },
            ..State::default()
        };
        parse_adif_char(&mut state, '>').unwrap();
        assert_eq!(state.field_state, FieldState::InValue);
        assert_eq!(state.adif_state, AdifState::InQso);
        assert_eq!(state.counter, 6);
        assert_eq!(state.buffer, "<CALL:6>");
    }

    #[test]
    fn test_parse_adif_char_qso_first_value_char() {
        let mut state = State {
            field_state: FieldState::InValue,
            adif_state: AdifState::InQso,
            counter: 6,
            buffer: "<CALL:6>".to_string(),
            adif: Adif {
                header: Header::try_from(vec![
                    HeaderField::new(
                        HeaderFieldName::PROGRAMID,
                        DataValue::String("test".to_string()),
                    ),
                    HeaderField::new(HeaderFieldName::EOH, DataValue::Null()),
                ])
                .unwrap(),
                ..Adif::default()
            },
            ..State::default()
        };
        parse_adif_char(&mut state, 'I').unwrap();
        assert_eq!(state.field_state, FieldState::InValue);
        assert_eq!(state.adif_state, AdifState::InQso);
        assert_eq!(state.counter, 5);
        assert_eq!(state.buffer, "<CALL:6>I");
    }

    #[test]
    fn test_parse_adif_char_qso_last_value_char() {
        let mut state = State {
            field_state: FieldState::InValue,
            adif_state: AdifState::InQso,
            counter: 1,
            buffer: "<CALL:6>IS0GV".to_string(),
            adif: Adif {
                header: Header::try_from(vec![
                    HeaderField::new(
                        HeaderFieldName::PROGRAMID,
                        DataValue::String("test".to_string()),
                    ),
                    HeaderField::new(HeaderFieldName::EOH, DataValue::Null()),
                ])
                .unwrap(),
                ..Adif::default()
            },
            ..State::default()
        };
        parse_adif_char(&mut state, 'H').unwrap();
        assert_eq!(state.field_state, FieldState::LookingForBeginning);
        assert_eq!(state.adif_state, AdifState::InQso);
        assert_eq!(state.counter, 0);
        assert_eq!(state.buffer, "");
        assert_eq!(state.qso.len(), 1);
    }

    #[test]
    fn test_parse_adif_char_qso_last_eor_char() {
        let mut state = State {
            field_state: FieldState::InTag,
            adif_state: AdifState::InQso,
            counter: 0,
            buffer: "<EOR".to_string(),
            qso: QSO::try_from(vec![QSOField::new(
                QSOFieldName::CALL,
                DataValue::String("IS0GVH".to_string()),
            )])
            .unwrap(),
            adif: Adif {
                header: Header::try_from(vec![
                    HeaderField::new(
                        HeaderFieldName::PROGRAMID,
                        DataValue::String("test".to_string()),
                    ),
                    HeaderField::new(HeaderFieldName::EOH, DataValue::Null()),
                ])
                .unwrap(),
                ..Adif::default()
            },
            ..State::default()
        };
        parse_adif_char(&mut state, '>').unwrap();
        assert_eq!(state.field_state, FieldState::LookingForBeginning);
        assert_eq!(state.adif_state, AdifState::InQso);
        assert_eq!(state.counter, 0);
        assert_eq!(state.buffer, "");
        assert_eq!(state.adif.qso.len(), 1);
        assert_eq!(state.qso.len(), 0);
    }
}
