use noa_parser::bytes::matchers::match_number;
use noa_parser::bytes::token::Token;
use noa_parser::errors::ParseResult;
use noa_parser::matcher::{Match, MatchSize};
use noa_parser::recognizer::{recognize, Recognizable};
use noa_parser::scanner::Scanner;
use noa_parser::visitor::Visitor;

#[derive(Debug)]
struct Addition {
    rhs: usize,
    lhs: usize,
    result: usize
}

struct TokenNumber;

impl Match<u8> for TokenNumber {
    fn matcher(&self, data: &[u8]) -> (bool, usize) {
        match_number(data)
    }
}

impl MatchSize for TokenNumber {
    fn size(&self) -> usize {
        0
    }
}

impl<'a> Recognizable<'a, u8, &'a [u8]> for TokenNumber {
    fn recognize(self, scanner: &mut Scanner<'a, u8>) -> ParseResult<Option<&'a [u8]>> {
        let (result, size) = scanner.recognize(self)?;
        if !result {
            return Ok(None)
        }
        let curent_position = scanner.current_position();
        if !scanner.is_empty() {
            scanner.bump_by(size);
        }
        Ok(Some(&scanner.data()[curent_position..curent_position + size]))
    }
}

struct Number(usize);

impl Visitor<'_, u8> for Number {
    fn accept(scanner: &mut Scanner<u8>) -> ParseResult<Self> {
        let raw_data = recognize(TokenNumber, scanner)?;
        let str_data = std::str::from_utf8(raw_data)?;
        let result = str_data.parse::<usize>()?;
        Ok(Number(result))
    }
}

impl<'a> Visitor<'a, u8> for Addition {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        let lhs = Number::accept(scanner)?.0;
        Token::Whitespace.recognize(scanner)?;
        Token::Plus.recognize(scanner)?;
        Token::Whitespace.recognize(scanner)?;
        let rhs = Number::accept(scanner)?.0;
        Token::Whitespace.recognize(scanner)?;
        Token::Equal.recognize(scanner)?;
        Token::Whitespace.recognize(scanner)?;
        let result = Number::accept(scanner)?.0;
        Ok(Addition { lhs, rhs, result })
    }
}

fn main() {
    let data = b"1 + 2 = 3";
    let mut scanner = Scanner::new(data);
    let result = Addition::accept(&mut scanner);
    println!("{:?}", result);
}