use crate::errors::{ParseError, ParseResult};
use crate::scanner::Scanner;
use crate::visitor::Visitor;
use std::marker::PhantomData;

pub struct SeparatedList<T, V, S> {
    pub(crate) data: Vec<V>,
    separator: PhantomData<(S, T)>,
}

enum YieldResult<V> {
    Last(V),
    MaybeNext(V),
}

impl<T, V, S> SeparatedList<T, V, S> {
    /// Consume the `SeparatedList` and return an iterator over the elements.
    ///
    /// # Returns
    ///
    /// An iterator over the elements of the `SeparatedList`.
    pub fn into_iter(self) -> impl Iterator<Item = V> {
        self.data.into_iter()
    }
}

/// Yield the next element in the list and tell if it's the last one.
///
/// # Type Parameters
///
/// * `T` - The type of the data to scan.
/// * `V` - The type of the element to yield.
/// * `S` - The type of the separator to consume.
///
/// # Arguments
///
/// * `scanner` - The scanner to use.
///
/// # Returns
///
/// A `YieldResult` containing the element and whether it's the last one.
///
/// # Errors
///
/// Any error the visitor for the element or the separator returns.
fn yield_element<'a, T, V, S>(scanner: &mut Scanner<'a, T>) -> ParseResult<YieldResult<V>>
where
    V: Visitor<'a, T>,
    S: Visitor<'a, T>,
{
    let cursor = scanner.current_position();
    let element = match scanner.visit::<V>() {
        Ok(element) => element,
        Err(err) => {
            scanner.jump_to(cursor);
            return Err(err);
        }
    };

    if scanner.remaining().is_empty() {
        return Ok(YieldResult::Last(element));
    }

    // consume the separator if not the end of the slice
    scanner.visit::<S>()?;

    Ok(YieldResult::MaybeNext(element))
}

impl<'a, T, V, S> Visitor<'a, T> for SeparatedList<T, V, S>
where
    V: Visitor<'a, T>,
    S: Visitor<'a, T>,
{
    /// Accept a list of elements separated by a separator.
    ///
    /// # Arguments
    ///
    /// * `scanner` - The scanner to use.
    ///
    /// # Returns
    ///
    /// A `ParseResult` containing the accepted `SeparatedList` on success, or
    /// an error on failure.
    ///
    /// # Errors
    ///
    /// Any error the visitor for the element or the separator returns, or
    /// `ParseError::UnexpectedToken` if the scanner is empty when attempting
    /// to parse the separator.
    fn accept(scanner: &mut Scanner<'a, T>) -> ParseResult<Self> {
        let mut elements = vec![];
        let cursor = scanner.current_position();

        loop {
            if let Ok(result) = yield_element::<T, V, S>(scanner) {
                let element: YieldResult<V> = result;

                match element {
                    YieldResult::Last(element) => {
                        elements.push(element);
                        break;
                    }
                    YieldResult::MaybeNext(element) => {
                        elements.push(element);
                    }
                }
            } else {
                scanner.jump_to(cursor);
                return Err(ParseError::UnexpectedToken);
            }
        }

        Ok(SeparatedList {
            data: elements,
            separator: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::bytes::primitives::number::Number;
    use crate::bytes::token::Token;
    use crate::errors::ParseResult;
    use crate::recognizer::recognize;
    use crate::scanner::Scanner;
    use crate::separated_list::SeparatedList;
    use crate::visitor::Visitor;

    struct SeparatorComma;

    impl<'a> Visitor<'a, u8> for SeparatorComma {
        fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
            recognize(Token::Comma, scanner)?;
            Ok(SeparatorComma)
        }
    }

    /// Tests parsing a list of `Number`s separated by commas.
    ///
    /// Input: `b"12,4,78,22"`
    /// Output: `vec![Number(12), Number(4), Number(78), Number(22)]`
    /// Final position: 10
    #[test]
    fn test_parse_number_list() {
        let data = b"12,4,78,22";
        let mut scanner = Scanner::new(data);
        let result = scanner
            .visit::<SeparatedList<u8, Number<usize>, SeparatorComma>>()
            .expect("failed to parse");
        assert_eq!(
            result.data,
            vec![Number(12), Number(4), Number(78), Number(22)]
        );
        assert_eq!(scanner.current_position(), 10);
    }
}
