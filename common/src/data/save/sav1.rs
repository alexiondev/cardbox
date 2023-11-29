use crate::data::save::Save;

#[derive(Debug)]
pub struct Sav1 {
    data: Vec<u8>
}

impl From<Vec<u8>> for Sav1 {
    fn from(data: Vec<u8>) -> Self {
        Self {
            data,
        }
    }
}
