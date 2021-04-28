#[derive(Debug)]
pub enum EncodingError {
    Unimplemented,
}

#[derive(Debug)]
pub struct Encoded(String);

pub fn encode(input: String) -> Result<Encoded, EncodingError> {
    Err(EncodingError::Unimplemented)
}
