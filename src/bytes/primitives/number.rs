//! Define the number token and its acceptor.

use crate::bytes::matchers::match_number;
use crate::errors::ParseResult;
use crate::matcher::{Match, MatchSize};
use crate::recognizer::recognize;
use crate::scanner::Scanner;
use crate::visitor::Visitor;

pub struct TokenNumber;

/// Implement the `Match` trait for the token number.
impl Match<u8> for TokenNumber {
    fn matcher(&self, data: &[u8]) -> (bool, usize) {
        match_number(data)
    }
}

/// Implement the `MatchSize` trait for the token number.
impl MatchSize for TokenNumber {
    fn size(&self) -> usize {
        0
    }
}

/// Define how to accept the token number.
#[derive(Debug, PartialEq)]
pub struct Number<T>(pub T);

/// Implement the `Visitor` trait for the token number.
macro_rules! impl_number {
    ($type:ty) => {
        impl Visitor<'_, u8> for Number<$type> {
            fn accept(scanner: &mut Scanner<u8>) -> ParseResult<Self> {
                let raw_data = recognize(TokenNumber, scanner)?;
                let str_data = std::str::from_utf8(raw_data)?;
                let result = str_data.parse::<$type>()?;
                Ok(Number(result))
            }
        }
    };
}

impl_number!(usize);
impl_number!(u8);
impl_number!(u16);
impl_number!(u32);
impl_number!(u64);
impl_number!(u128);
impl_number!(isize);
impl_number!(i8);
impl_number!(i16);
impl_number!(i32);
impl_number!(i64);
impl_number!(i128);
