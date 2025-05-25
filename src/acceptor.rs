use crate::errors::{ParseError, ParseResult};
use crate::scanner::Scanner;
use crate::visitor::Visitor;

/// A type that wraps a `Scanner` and holds a successfully accepted value.
///
/// When a value is successfully accepted, the `Acceptor` stores the value in its
/// `data` field and returns itself. If a value is not accepted, the `Acceptor`
/// rewinds the scanner to the previous position and returns itself.
///
/// # Type Parameters
///
/// * `T` - The type of the data to scan.
/// * `V` - The type of the value to accept.
/// * `'a` - The lifetime of the data to scan.
/// * `'b` - The lifetime of the acceptor.
#[derive(Debug)]
pub struct Acceptor<'a, 'b, T, V> {
    /// The accepted value, if any.
    data: Option<V>,
    /// The scanner to use when consuming input.
    scanner: &'b mut Scanner<'a, T>,
}

impl<'a, 'b, T, V> Acceptor<'a, 'b, T, V> {
    /// Create a new acceptor.
    ///
    /// # Arguments
    ///
    /// * `scanner` - The scanner to use when consuming input.
    ///
    /// # Returns
    ///
    /// A new acceptor that uses the given scanner.
    pub fn new(scanner: &'b mut Scanner<'a, T>) -> Acceptor<'a, 'b, T, V> {
        Acceptor {
            data: None,
            scanner,
        }
    }
}

impl<'a, T, V> Acceptor<'a, '_, T, V> {
    /// Attempt to accept a `U` using the given `transformer`, and rewind the scanner
    /// and return the current acceptor if it fails.
    ///
    /// # Arguments
    ///
    /// * `transformer` - A function that takes a `U` and returns a `ParseResult<V>`.
    ///
    /// # Returns
    ///
    /// If the `U` is successfully accepted and the `transformer` returns `Ok`, returns
    /// the current acceptor with the resulting value in `data`. If the `U` is not
    /// successfully accepted, returns the current acceptor with the current position
    /// of the scanner rewound to the position at which the `U` was attempted, and
    /// `data` is left `None`.
    pub fn try_or<U: Visitor<'a, T>, F>(mut self, transformer: F) -> ParseResult<Self>
    where
        F: Fn(U) -> V,
    {
        let cursor = self.scanner.current_position();
        // Propagate the data
        if self.data.is_some() {
            return Ok(self);
        }

        match U::accept(self.scanner) {
            Ok(found) => {
                self.data = Some(transformer(found));
            }
            Err(ParseError::UnexpectedToken) => {
                self.scanner.jump_to(cursor);
            }
            Err(err) => {
                return Err(err);
            }
        }

        Ok(self)
    }

    /// Consume the acceptor and return the `V` that was accepted if the acceptor was
    /// successful.
    ///
    /// # Returns
    ///
    /// If the acceptor was successful (i.e., `data` is `Some`), returns the `V` that
    /// was accepted. Otherwise, returns `None`.
    pub fn finish(self) -> Option<V> {
        self.data
    }
}
