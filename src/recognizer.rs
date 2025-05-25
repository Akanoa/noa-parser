use crate::errors::{ParseError, ParseResult};
use crate::matcher::{Match, MatchSize};
use crate::scanner::Scanner;

/// Describes a recognizable object.
pub trait Recognizable<'a, T, V>: MatchSize {
    /// Try to recognize the object for the given scanner.
    ///
    /// # Type Parameters
    /// V - The type of the object to recognize
    ///
    /// # Arguments
    /// * `scanner` - The scanner to recognize the object for.
    ///
    /// # Returns
    /// * `Ok(Some(V))` if the object was recognized,
    /// * `Ok(None)` if the object was not recognized,
    /// * `Err(ParseError)` if an error occurred
    ///
    fn recognize(self, scanner: &mut Scanner<'a, T>) -> ParseResult<Option<V>>;
}

/// Recognize an object for the given scanner.
///
/// # Type Parameters
/// * `V` - The type of the object to recognize
/// * `R` - The type of the recognizable object
///
/// # Arguments
/// * `recognizable` - The recognizable object to use for recognition
/// * `scanner` - The scanner to recognize the object for
///
/// # Returns
/// * `Ok(V)` if the object was recognized,
/// * `Err(ParseError)` if an error occurred
///
/// This function calls the `recognize` method of the recognizable object and
/// returns its result. If the recognizable object was not recognized, an
/// `Err(ParseError::UnexpectedToken)` is returned. If the scanner is at the end
/// of its input and the recognizable object is longer than the remaining input,
/// an `Err(ParseError::UnexpectedEndOfInput)` is returned.
pub fn recognize<'a, T, V, R: Recognizable<'a, T, V>>(
    recognizable: R,
    scanner: &mut Scanner<'a, T>,
) -> ParseResult<V> {
    if recognizable.size() > scanner.remaining().len() {
        return Err(ParseError::UnexpectedEndOfInput);
    }
    recognizable
        .recognize(scanner)?
        .ok_or(ParseError::UnexpectedToken)
}

/// Recognize an object for the given scanner.
/// Return a slice of the recognized object.
impl<'a, T, M: Match<T> + MatchSize> Recognizable<'a, T, &'a [T]> for M {
    fn recognize(self, scanner: &mut Scanner<'a, T>) -> ParseResult<Option<&'a [T]>> {

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