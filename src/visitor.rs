use crate::errors::ParseResult;
use crate::scanner::Scanner;

/// A `Visitor` is a trait that allows to define how to visit a `Scanner`.
///
/// When a `Visitor` is used on a `Scanner`, it will consume the input from the
/// scanner and return the result of the visit.
///
/// # Type Parameters
///
/// * `T` - The type of the data to visit.
///
/// # Associated Functions
///
/// * `accept` - Try to accept the `Scanner` and return the result of the visit.
pub trait Visitor<'a, T>: Sized {
    /// Try to accept the `Scanner` and return the result of the visit.
    ///
    /// # Arguments
    ///
    /// * `scanner` - The scanner to accept.
    ///
    /// # Returns
    ///
    /// The result of the visit.
    fn accept(scanner: &mut Scanner<'a, T>) -> ParseResult<Self>;
}
