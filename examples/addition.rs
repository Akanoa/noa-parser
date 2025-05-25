use noa_parser::bytes::matchers::match_number;
use noa_parser::bytes::token::Token;
use noa_parser::errors::ParseResult;
use noa_parser::matcher::{Match, MatchSize};
use noa_parser::recognizer::{Recognizable, recognize};
use noa_parser::scanner::Scanner;
use noa_parser::visitor::Visitor;

/// The token number which recognizes numbers.
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

/// Define how to accept the token number.
struct Number(usize);

/// Implement the `Visitor` trait for the token number.
impl Visitor<'_, u8> for Number {
    fn accept(scanner: &mut Scanner<u8>) -> ParseResult<Self> {
        let raw_data = recognize(TokenNumber, scanner)?;
        let str_data = std::str::from_utf8(raw_data)?;
        let result = str_data.parse::<usize>()?;
        Ok(Number(result))
    }
}

/// Define the addition expression.
#[derive(Debug)]
#[allow(dead_code)]
struct Addition {
    rhs: usize,
    lhs: usize,
    result: usize,
}

/// Implement the `Visitor` trait for the addition expression.
impl<'a> Visitor<'a, u8> for Addition {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        // Parse the first number
        let lhs = Number::accept(scanner)?.0;
        // Expect a whitespace and a plus token then whitespace
        Token::Whitespace.recognize(scanner)?;
        Token::Plus.recognize(scanner)?;
        Token::Whitespace.recognize(scanner)?;
        // Parse the second number
        let rhs = Number::accept(scanner)?.0;
        // Expect a whitespace and an equal token then whitespace
        Token::Whitespace.recognize(scanner)?;
        Token::Equal.recognize(scanner)?;
        Token::Whitespace.recognize(scanner)?;
        // Parse the result number
        let result = Number::accept(scanner)?.0;
        // Return the addition
        Ok(Addition { lhs, rhs, result })
    }
}

fn main() {
    let data = b"1 + 2 = 3";
    let mut scanner = Scanner::new(data);
    let result = Addition::accept(&mut scanner);
    println!("{:?}", result);
}
