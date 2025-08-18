use crate::data::AdifData;
use crate::header::HeaderField;
use crate::qso::QSOField;
use std::fmt::{Debug, Display, Formatter};

pub trait AdifItem: AdifData + Debug + Clone + PartialEq {
    fn add_end_if_missing(&self) -> Self;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Adif {
    pub headers: Vec<HeaderField>,
    pub qso: Vec<QSOField>,
}

impl AdifData for Adif {
    fn serialize(&self) -> String {
        let headers = self
            .headers
            .iter()
            .map(HeaderField::serialize)
            .chain(std::iter::once("<EOH>".to_string()));

        let qso = self.qso.iter().map(QSOField::serialize);

        headers.chain(qso).collect::<Vec<String>>().join("\n")
    }

    fn deserialize(value: &str) -> crate::result::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Display for Adif {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Headers: {} - QSO: {}",
            self.headers.len(),
            self.qso.len()
        )
    }
}
