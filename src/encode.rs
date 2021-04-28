#[derive(Debug)]
pub enum EncodingError {
    Unimplemented,
}

#[derive(Debug)]
pub struct Encoded(String);

pub fn encode(input: String, lookback_limit: usize) -> Result<Encoded, EncodingError> {
    let input_len = input.len();
    let mut starting_position: usize = 0;
    let search_buffer: Vec<char> = Vec::new();
    let encoded = Vec::new();

    while starting_position < input_len {
        let next_match = find_next_match(input, search_buffer, lookback_limit).unwrap();
        starting_position += next_match.1 + 1;
        encoded.push(next_match);
    }

    Err(EncodingError::Unimplemented)
}

fn find_next_match(
    input: String,
    search_buffer: Vec<char>,
    lookback_limit: usize,
) -> Option<(usize, usize, char)> {
    let offset: Option<usize> = None; // negative offset to start of match
    let length: Option<usize> = None; // length of match
    let character: Option<char> = None; // character after match
    character.and_then(|c| length.and_then(|l| offset.and_then(|o| Some((o, l, c)))))
}
