//! Recognize whitespaces

use crate::bytes::token::Token;
use crate::errors::{ParseError, ParseResult};
use crate::recognizer::Recognizable;
use crate::scanner::Scanner;
use crate::visitor::Visitor;

/// Recognize at least one whitespace
pub struct Whitespaces;

/// Recognize zero or more whitespaces
pub struct OptionalWhitespaces;

impl<'a> Visitor<'a, u8> for Whitespaces {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        let mut found = false;
        while Token::Whitespace.recognize(scanner)?.is_some() {
            found = true;
        }
        if !found {
            return Err(ParseError::UnexpectedToken);
        }
        Ok(Whitespaces)
    }
}

impl<'a> Visitor<'a, u8> for OptionalWhitespaces {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        if scanner.is_empty() {
            return Ok(OptionalWhitespaces);
        }
        while Token::Whitespace.recognize(scanner)?.is_some() {}
        Ok(OptionalWhitespaces)
    }
}
