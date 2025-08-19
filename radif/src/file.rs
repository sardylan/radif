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
use futures::io::BufReader;
use futures::{AsyncRead, AsyncReadExt, TryStreamExt};

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

#[derive(Debug, PartialEq)]
struct State {
    field_state: FieldState,
    adif_state: AdifState,
    buffer: String,
    counter: isize,
    qso: QSO,
    adif: Adif,
}

impl Default for State {
    fn default() -> Self {
        State {
            field_state: FieldState::LookingForBeginning,
            adif_state: AdifState::InHeader,
            buffer: String::new(),
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
    Ok(async_stream::stream! {
        // let mut reader = BufReader::new(reader);
        let mut char = [0u8;1];
        loop {
            match reader.read(&mut char).await {
                Ok(0) => break,
                Ok(_) => yield Ok(char[0]),
                Err(e) => {
                    yield Err(DeserializeError(e.to_string()));
                    break;
                }
            }
        }
    }
    .and_then(|c| async move { Ok(c as char) })
    .try_fold(State::default(), parse_adif_char)
    .await?
    .adif)
}

async fn parse_adif_char(state: State, c: char) -> result::Result<State> {
    match state.field_state {
        FieldState::LookingForBeginning => match c {
            '<' => Ok(State {
                field_state: FieldState::InTag,
                buffer: "<".to_string(),
                counter: 0,
                ..state
            }),
            _ => Ok(state),
        },
        FieldState::InTag => match c {
            '>' => {
                let buffer = format!("{}>", &state.buffer);
                match state.adif_state {
                    AdifState::InHeader => {
                        if buffer == HeaderField::end().serialize() {
                            return Ok(State {
                                field_state: FieldState::LookingForBeginning,
                                adif_state: AdifState::InQso,
                                buffer: String::new(),
                                counter: 0,
                                adif: Adif {
                                    header: state.adif.header.add_field(&HeaderField::end()),
                                    ..state.adif
                                },
                                ..state
                            });
                        }
                    }
                    AdifState::InQso => {
                        if buffer == QSOField::end().serialize() {
                            return Ok(State {
                                field_state: FieldState::LookingForBeginning,
                                buffer: String::new(),
                                counter: 0,
                                adif: Adif {
                                    qso: state
                                        .adif
                                        .qso
                                        .into_iter()
                                        .chain(std::iter::once(
                                            state.qso.add_field(&QSOField::end()),
                                        ))
                                        .collect(),
                                    ..state.adif
                                },
                                qso: QSO::default(),
                                ..state
                            });
                        }
                    }
                }

                Ok(State {
                    field_state: FieldState::InValue,
                    buffer,
                    counter: state
                        .buffer
                        .split(":")
                        .last()
                        .ok_or_else(|| DeserializeError("Bad buffer".to_string()))?
                        .parse::<usize>()
                        .map_err(|e| DeserializeError(e.to_string()))?
                        as isize,
                    ..state
                })
            }
            c => Ok(State {
                buffer: format!("{}{}", &state.buffer, c),
                ..state
            }),
        },
        FieldState::InValue => {
            let buffer = format!("{}{}", &state.buffer, c);

            if state.counter > 1 {
                return Ok(State {
                    buffer,
                    counter: state.counter - 1,
                    ..state
                });
            }

            match state.adif_state {
                AdifState::InHeader => {
                    let header_field = HeaderField::deserialize(&buffer)?;
                    Ok(State {
                        field_state: FieldState::LookingForBeginning,
                        adif_state: if header_field.is_end() {
                            AdifState::InQso
                        } else {
                            state.adif_state
                        },
                        buffer: String::new(),
                        counter: 0,
                        adif: Adif {
                            header: state.adif.header.add_field(&header_field),
                            ..state.adif
                        },
                        ..state
                    })
                }
                AdifState::InQso => {
                    let qso_field = QSOField::deserialize(&buffer)?;
                    let qso = state.qso.add_field(&qso_field);
                    Ok(State {
                        field_state: FieldState::LookingForBeginning,
                        buffer: String::new(),
                        counter: 0,
                        adif: if qso_field.is_end() {
                            Adif {
                                qso: state
                                    .adif
                                    .qso
                                    .into_iter()
                                    .chain(std::iter::once(qso.clone()))
                                    .collect(),
                                ..state.adif
                            }
                        } else {
                            state.adif
                        },
                        qso: if !qso_field.is_end() {
                            qso
                        } else {
                            QSO::default()
                        },
                        ..state
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::data::DataValue;
    use crate::fields::header::HeaderFieldName;
    use crate::fields::qso::QSOFieldName;
    use crate::header::Header;
    use futures::executor::block_on;

    #[test]
    fn test_parse_adif_char_default() {
        let input_state = State::default();
        let input_char = ' ';
        let expected = State::default();
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);

        let input_state = State::default();
        let input_char = '>';
        let expected = State::default();
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);

        let input_state = State::default();
        let input_char = ':';
        let expected = State::default();
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_adif_char_header_first_char() {
        let input_state = State::default();
        let input_char = '<';
        let expected = State {
            field_state: FieldState::InTag,
            buffer: "<".to_string(),
            ..State::default()
        };
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_adif_char_header_second_char() {
        let input_state = State {
            field_state: FieldState::InTag,
            buffer: "<".to_string(),
            ..State::default()
        };
        let input_char = 'A';
        let expected = State {
            field_state: FieldState::InTag,
            buffer: "<A".to_string(),
            ..State::default()
        };
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_adif_char_header_separator() {
        let input_state = State {
            field_state: FieldState::InTag,
            buffer: "<PROGRAMID".to_string(),
            ..State::default()
        };
        let input_char = ':';
        let expected = State {
            field_state: FieldState::InTag,
            buffer: "<PROGRAMID:".to_string(),
            ..State::default()
        };
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_adif_char_header_value_begin() {
        let input_state = State {
            field_state: FieldState::InTag,
            buffer: "<PROGRAMID:4".to_string(),
            ..State::default()
        };
        let input_char = '>';
        let expected = State {
            field_state: FieldState::InValue,
            counter: 4,
            buffer: "<PROGRAMID:4>".to_string(),
            ..State::default()
        };
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_adif_char_header_value_first_char() {
        let input_state = State {
            field_state: FieldState::InValue,
            counter: 4,
            buffer: "<PROGRAMID:4>".to_string(),
            ..State::default()
        };
        let input_char = 't';
        let expected = State {
            field_state: FieldState::InValue,
            counter: 3,
            buffer: "<PROGRAMID:4>t".to_string(),
            ..State::default()
        };
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_adif_char_header_value_last_char() {
        let input_state = State {
            field_state: FieldState::InValue,
            counter: 1,
            buffer: "<PROGRAMID:4>tes".to_string(),
            ..State::default()
        };
        let input_char = 't';
        let expected = State {
            field_state: FieldState::LookingForBeginning,
            counter: 0,
            buffer: "".to_string(),
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
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_adif_char_header_eoh_last_char() {
        let input_state = State {
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
        let input_char = '>';
        let expected = State {
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
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_adif_char_qso_first_char() {
        let input_state = State {
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
        let input_char = '<';
        let expected = State {
            field_state: FieldState::InTag,
            adif_state: AdifState::InQso,
            counter: 0,
            buffer: "<".to_string(),
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
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_adif_char_qso_second_char() {
        let input_state = State {
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
        let input_char = ':';
        let expected = State {
            field_state: FieldState::InTag,
            adif_state: AdifState::InQso,
            counter: 0,
            buffer: "<CALL:".to_string(),
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
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_adif_char_qso_last_tag_char() {
        let input_state = State {
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
        let input_char = '>';
        let expected = State {
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
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_adif_char_qso_fisrt_value_char() {
        let input_state = State {
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
        let input_char = 'I';
        let expected = State {
            field_state: FieldState::InValue,
            adif_state: AdifState::InQso,
            counter: 5,
            buffer: "<CALL:6>I".to_string(),
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
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_adif_char_qso_last_value_char() {
        let input_state = State {
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
        let input_char = 'H';
        let expected = State {
            field_state: FieldState::LookingForBeginning,
            adif_state: AdifState::InQso,
            counter: 0,
            buffer: "".to_string(),
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
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_adif_char_qso_last_eor_char() {
        let input_state = State {
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
        let input_char = '>';
        let expected = State {
            field_state: FieldState::LookingForBeginning,
            adif_state: AdifState::InQso,
            counter: 0,
            buffer: "".to_string(),
            qso: QSO::default(),
            adif: Adif {
                header: Header::try_from(vec![
                    HeaderField::new(
                        HeaderFieldName::PROGRAMID,
                        DataValue::String("test".to_string()),
                    ),
                    HeaderField::new(HeaderFieldName::EOH, DataValue::Null()),
                ])
                .unwrap(),
                qso: vec![QSO::try_from(vec![
                    QSOField::new(QSOFieldName::CALL, DataValue::String("IS0GVH".to_string())),
                    QSOField::new(QSOFieldName::EOR, DataValue::Null()),
                ])
                .unwrap()],
            },
            ..State::default()
        };
        let actual = block_on(parse_adif_char(input_state, input_char)).unwrap();
        assert_eq!(actual, expected);
    }
}
