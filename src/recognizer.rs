//! Defines how to recognize an object.

use crate::errors::{ParseError, ParseResult};
use crate::matcher::{Match, MatchSize};
use crate::scanner::Scanner;

/// A trait that defines how to recognize an object.
///
/// # Type Parameters
/// * `V` - The type of the object to recognize
/// * `T` - The type of the data to scan
/// * `'a` - The lifetime of the data to scan
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

pub trait RecognizeSelf<'a, T, V>: MatchSize {
    /// Try to recognize the object for the given scanner.
    ///
    /// # Arguments
    /// * `scanner` - The scanner to recognize the object for.
    ///
    /// # Returns
    /// * `Ok(Some(V))` if the object was recognized,
    /// * `Ok(None)` if the object was not recognized,
    /// * `Err(ParseError)` if an error occurred
    fn recognize_self(self, scanner: &mut Scanner<'a, T>) -> ParseResult<Option<V>>;
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
impl<'a, T, M: Match<T> + MatchSize> RecognizeSelf<'a, T, M> for M {
    fn recognize_self(self, scanner: &mut Scanner<'a, T>) -> ParseResult<Option<M>> {
        if scanner.is_empty() {
            return Err(ParseError::UnexpectedEndOfInput);
        }

        let data = scanner.remaining();

        let (result, size) = self.matcher(data);
        if !result {
            return Ok(None);
        }
        if !scanner.is_empty() {
            scanner.bump_by(size);
        }
        Ok(Some(self))
    }
}

/// A `Recognizer` is a type that wraps a `Scanner` and holds a successfully
/// recognized value.
///
/// When a value is successfully recognized, the `Recognizer` stores the value in
/// its `data` field and returns itself. If a value is not recognized, the
/// `Recognizer` rewinds the scanner to the previous position and returns itself.
///
/// # Type Parameters
///
/// * `T` - The type of the data to scan.
/// * `U` - The type of the value to recognize.
/// * `'a` - The lifetime of the data to scan.
/// * `'container` - The lifetime of the `Recognizer`.
pub struct Recognizer<'a, 'container, T, U> {
    data: Option<U>,
    scanner: &'container mut Scanner<'a, T>,
}

impl<'a, 'b, T, R: RecognizeSelf<'a, T, R>> Recognizer<'a, 'b, T, R> {
    /// Create a new `Recognizer` with the given scanner.
    ///
    /// # Arguments
    ///
    /// * `scanner` - The scanner to use when recognizing input.
    ///
    /// # Returns
    ///
    /// A new `Recognizer` that uses the given scanner.
    pub fn new(scanner: &'b mut Scanner<'a, T>) -> Self {
        Recognizer {
            data: None,
            scanner,
        }
    }

    /// Attempt to recognize a `U` using the given `element`, and return the
    /// current recognizer if it fails.
    ///
    /// # Arguments
    ///
    /// * `element` - A `Recognizable` that recognizes a `U`.
    ///
    /// # Returns
    ///
    /// If the `U` is successfully recognized, returns the current recognizer with
    /// the resulting value in `data`. If the `U` is not successfully recognized,
    /// returns the current recognizer with the current position of the scanner
    /// rewound to the position at which the `U` was attempted, and `data` is left
    /// `None`.
    pub fn try_or(mut self, element: R) -> ParseResult<Recognizer<'a, 'b, T, R>> {
        // Propagate result
        if self.data.is_some() {
            return Ok(self);
        }
        // Or apply current recognizer
        if let Some(found) = element.recognize_self(self.scanner)? {
            self.data = Some(found);
        }
        Ok(self)
    }

    /// Consume the recognizer and return the `U` that was recognized if the
    /// recognizer was successful.
    ///
    /// # Returns
    ///
    /// If the recognizer was successful (i.e., `data` is `Some`), returns the
    /// `U` that was recognized. Otherwise, returns `None`.
    pub fn finish(self) -> Option<R> {
        self.data
    }

    /// Consume the recognizer and return the `U` that was recognized if the
    /// recognizer was successful, or run the given closure if the recognizer was
    /// not successful.
    ///
    /// # Arguments
    ///
    /// * `closure` - A function that takes the `Scanner` and returns a
    ///   `ParseResult<U>`.
    ///
    /// # Returns
    ///
    /// If the recognizer was successful (i.e., `data` is `Some`), returns the
    /// `U` that was recognized. If the recognizer was not successful, the
    /// `closure` is called with the `Scanner` and the result of the closure is
    /// returned.
    pub fn finish_with<F>(self, closure: F) -> ParseResult<R>
    where
        F: FnOnce(&mut Scanner<'a, T>) -> ParseResult<R>,
    {
        match self.data {
            None => closure(self.scanner),
            Some(token) => Ok(token),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bytes::token::Token;
    use crate::errors::ParseResult;
    use crate::recognizer::{RecognizeSelf, Recognizer};

    #[test]
    fn test_recognizer() {
        let data = b">";
        let mut scanner = crate::scanner::Scanner::new(data);
        let result = Token::GreaterThan
            .recognize_self(&mut scanner)
            .expect("failed to parse");
        assert_eq!(result, Some(Token::GreaterThan));
    }

    #[test]
    fn test_recognizer_multiple() -> ParseResult<()> {
        let data = b">>";
        let mut scanner = crate::scanner::Scanner::new(data);
        let result = Recognizer::new(&mut scanner)
            .try_or(Token::LessThan)?
            .try_or(Token::GreaterThan)?
            .finish()
            .expect("failed to parse");
        assert_eq!(result, Token::GreaterThan);
        Ok(())
    }
}
