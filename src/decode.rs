use crate::encode::Encoded;
use std::str::{from_utf8, Utf8Error};

#[derive(Debug)]
pub enum DecodingError {
    UTF8Error(Utf8Error),
    Unimplemented,
}

pub fn decode(input: Encoded) -> Result<String, DecodingError> {
    let mut decoded: Vec<u8> = vec![];

    for (o, l, c) in input.0 {
        println!("decoding ({:?}, {:?}, {:?})", l, o, c);

        let curr_len = decoded.len();
        let str_start = curr_len - o;
        println!("curr_len - o: {:?} - {:?} = {:?}", curr_len, o, str_start);
        let str_end = curr_len - o + l;
        println!(
            "reused from {:?} to {:?} of {:?}",
            str_start, str_end, curr_len
        );
        let reused_str = &decoded[str_start..str_end].to_vec();
        println!("reused_str: {:?}", reused_str);
        decoded.extend(reused_str);
        println!("c: {:?}", c);
        decoded.push(c.clone());
        println!("new decoded: {:?}", decoded);
    }

    from_utf8(&decoded)
        .map(|s| s.to_string())
        .map_err(DecodingError::UTF8Error)
}
