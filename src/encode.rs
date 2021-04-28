#[derive(Debug)]
pub enum EncodingError {
    Unimplemented,
}

#[derive(Debug)]
pub struct Encoded(String);

pub fn encode(
    input: &str,
    search_buffer_size: usize,
    lookahead_buffer_size: usize,
) -> Result<Encoded, EncodingError> {
    let input_len = input.len();
    let mut starting_position: usize = 0;
    let mut encoded = Vec::new();

    while starting_position < input_len {
        let next_match = find_next_match(input, search_buffer_size, lookahead_buffer_size).unwrap();
        starting_position += next_match.1 + 1;
        encoded.push(next_match);
    }

    Err(EncodingError::Unimplemented)
}

fn find_next_match(
    input: &str,
    search_buffer_size: usize,
    lookahead_buffer_size: usize,
) -> Option<(usize, usize, char)> {
    let offset: Option<usize> = None; // negative offset to start of match
    let length: Option<usize> = None; // length of match
    let character: Option<char> = None; // character after match
    character.and_then(|c| length.and_then(|l| offset.and_then(|o| Some((o, l, c)))))
}
