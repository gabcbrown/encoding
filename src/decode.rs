use crate::encode::Encoded;
use std::str::{from_utf8, Utf8Error};

#[derive(Debug)]
pub enum DecodingError {
    UTF8Error(Utf8Error),
}

pub fn decode(input: Encoded) -> Result<String, DecodingError> {
    let mut decoded: Vec<u8> = vec![];

    // (offset, length, character)
    for (o, l, c) in input.0 {
        let len = decoded.len();
        decoded.extend(&decoded[len - o as usize..len - o as usize + l as usize].to_vec());
        decoded.push(c.clone());
    }

    from_utf8(&decoded)
        .map(|s| s.to_string())
        .map_err(DecodingError::UTF8Error)
}
