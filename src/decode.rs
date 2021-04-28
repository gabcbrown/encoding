use crate::encode::Encoded;

#[derive(Debug)]
pub enum DecodingError {
    Unimplemented,
}

pub fn decode(input: Encoded) -> Result<String, DecodingError> {
    Err(DecodingError::Unimplemented)
}
