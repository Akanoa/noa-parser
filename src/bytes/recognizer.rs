use crate::errors::{ParseError, ParseResult};
use crate::matcher::{Match, MatchSize};
use crate::recognizer::Recognizable;
use crate::scanner::Scanner;

/// Recognize an object for the given scanner.
/// Return a slice of the recognized object.
impl<'a, M: Match<u8> + MatchSize> Recognizable<'a, u8, &'a [u8]> for M {
    fn recognize(self, scanner: &mut Scanner<'a, u8>) -> ParseResult<Option<&'a [u8]>> {
        if scanner.is_empty() {
            return Err(ParseError::UnexpectedEndOfInput);
        }

        let data = scanner.remaining();

        let (result, size) = self.matcher(data);
        if !result {
            return Ok(None);
        }
        let curent_position = scanner.current_position();
        if !scanner.is_empty() {
            scanner.bump_by(size);
        }
        Ok(Some(
            &scanner.data()[curent_position..curent_position + size],
        ))
    }
}
