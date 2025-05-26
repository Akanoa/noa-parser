use crate::bytes::token::Token;
use crate::errors::ParseResult;
use crate::peek::{PeekResult, Peekable, UntilEnd};
use crate::scanner::Scanner;

impl<'a> Peekable<'a, u8, Token, Token> for UntilEnd<u8> {
    /// Peeks at the current position of the `Scanner` until it reaches the end
    /// of the data.
    ///
    /// # Arguments
    ///
    /// * `data` - The `Scanner` to use when matching.
    ///
    /// # Returns
    ///
    /// A `PeekResult` where the `end_slice` is the current position of the
    /// `Scanner`, and `start` and `end` are both `()`.
    fn peek(&self, data: &Scanner<'a, u8>) -> ParseResult<PeekResult<Token, Token>> {
        Ok(PeekResult::Found {
            end_slice: data.remaining().len(),
            start: Token::Whitespace,
            end: Token::Whitespace,
        })
    }
}
