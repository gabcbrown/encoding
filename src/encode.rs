use std::cmp::min;

#[derive(Debug)]
pub enum EncodingError {
    UnexpectedEndOfString,
}

#[derive(Debug)]
pub struct Encoded(pub Vec<(usize, usize, u8)>);

pub fn encode(
    input: &str,
    search_buffer_size: usize,
    lookahead_buffer_size: usize,
) -> Result<Encoded, EncodingError> {
    let bytes = input.as_bytes();
    let input_len = bytes.len();
    let mut starting_index: usize = 0;
    let mut encoded = Vec::new();

    // TODO: verify what happens in edge case starting pos = input len... etc.
    while starting_index < input_len {
        let next_match = find_next_match(
            bytes,
            starting_index,
            search_buffer_size,
            lookahead_buffer_size,
        )
        .ok_or(EncodingError::UnexpectedEndOfString)?; // TODO: correct error?
        starting_index += next_match.1 + 1;
        encoded.push(next_match);
    }

    Ok(Encoded(encoded))
}

// TODO: need utf-e encoding support? if slice hits middle of multicharacter encoding we'll have a problem
fn find_next_match(
    input: &[u8],
    starting_index: usize,
    search_buffer_size: usize,
    lookahead_buffer_size: usize,
) -> Option<(usize, usize, u8)> {
    let buffer_window = min(starting_index, search_buffer_size);
    let lookahead_window = min(input.len() - starting_index - 1, lookahead_buffer_size); // - 1 because length and index are are off by 1 from each other, and we need to know the indexes available

    // +1 because .. upper bound is exclusive
    for j in (1..min(buffer_window, lookahead_window) + 1).rev() {
        if let Some(matched) = search_from_end(
            &input[starting_index - buffer_window..starting_index],
            &input[starting_index..starting_index + j],
        ) {
            return Some((
                buffer_window - matched,
                j,
                input[starting_index + j].clone(),
            ));
        }
    }

    // if we get here, there was no match
    Some((0, 0, input[starting_index].clone()))
}

fn search_from_end(input1: &[u8], input2: &[u8]) -> Option<usize> {
    if input1.len() < input2.len() {
        None
    } else {
        for i in (0..input1.len() - input2.len() + 1).rev() {
            if input1[i..i + input2.len()] == input2[..] {
                return Some(i);
            }
        }
        None
    }
}
