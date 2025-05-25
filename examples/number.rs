use noa_parser::bytes::matchers::match_number;
use noa_parser::matcher::{Match, MatchSize};
use noa_parser::recognizer::Recognizable;

struct TokenNumber;

/// Implement the `Match` trait for the token number.
impl Match<u8> for TokenNumber {
    fn matcher(&self, data: &[u8]) -> (bool, usize) {
        match_number(data)
    }
}

/// Implement the `MatchSize` trait for the token number.
impl MatchSize for TokenNumber {
    fn size(&self) -> usize {
        0
    }
}

fn main() {
    let data = b"123abc";
    let mut scanner = noa_parser::scanner::Scanner::new(data);
    let result = TokenNumber.recognize(&mut scanner);
    println!("{:?}", result); // Ok(Some([49, 50, 51]))
    // If the result is successful
    if let Ok(Some(data)) = result {
        // Convert the data to a string
        let str_data = std::str::from_utf8(data).unwrap();
        // Convert the string to a number
        let result = str_data.parse::<usize>().unwrap();
        println!("{}", result); // 123
    }
}