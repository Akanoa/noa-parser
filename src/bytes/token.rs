use crate::bytes::matchers::match_char;
use crate::errors::ParseResult;
use crate::matcher::{Match, MatchSize};
use crate::recognizer::Recognizable;
use crate::scanner::Scanner;

/// The token type
pub enum Token {
    /// The "(" character
    OpenParen,
    /// The `)` character
    CloseParen,
    /// The `,` character
    Comma,
    /// The `;` character
    Semicolon,
    /// The `:` character
    Colon,
    /// The whitespace character
    Whitespace,
    /// The `>` character
    GreaterThan,
    /// The `<` character
    LessThan,
    /// The `!` character
    Exclamation,
    /// The `'` character
    Quote,
    /// The `"` character
    DoubleQuote,
    /// The `=` character
    Equal,
    /// The `+` character
    Plus,
}

impl Match<u8> for Token {
    fn matcher(&self, data: &[u8]) -> (bool, usize) {
        match self {
            Token::OpenParen => match_char('(', data),
            Token::CloseParen => match_char(')', data),
            Token::Comma => match_char(',', data),
            Token::Semicolon => match_char(';', data),
            Token::Colon => match_char(':', data),
            Token::Whitespace => match_char(' ', data),
            Token::GreaterThan => match_char('>', data),
            Token::LessThan => match_char('<', data),
            Token::Exclamation => match_char('!', data),
            Token::Quote => match_char('\'', data),
            Token::DoubleQuote => match_char('"', data),
            Token::Equal => match_char('=', data),
            Token::Plus => match_char('+', data),
        }
    }
}

impl MatchSize for Token {
    fn size(&self) -> usize {
        match self {
            Token::OpenParen => 1,
            Token::CloseParen => 1,
            Token::Comma => 1,
            Token::Semicolon => 1,
            Token::Colon => 1,
            Token::Whitespace => 1,
            Token::GreaterThan => 1,
            Token::LessThan => 1,
            Token::Exclamation => 1,
            Token::Quote => 1,
            Token::DoubleQuote => 1,
            Token::Equal => 1,
            Token::Plus => 1,
        }
    }
} 

impl<'a> Recognizable<'a, u8, &'a [u8]> for Token {
    fn recognize(self, scanner: &mut Scanner<'a, u8>) -> ParseResult<Option<&'a [u8]>> {
        let (result, size) = scanner.recognize(self)?;
        if !result {
            return Ok(None)
        }
        let current_position = scanner.current_position();
        if !scanner.is_empty() {
            scanner.bump_by(size);
        }
        Ok(Some(&scanner.data()[current_position..current_position + size]))
    }
}
