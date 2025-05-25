//! Binary operators
use crate::acceptor::Acceptor;
use crate::bytes::token::Token;
use crate::errors::{ParseError, ParseResult};
use crate::recognizer::recognize;
use crate::scanner::Scanner;
use crate::visitor::Visitor;

enum BinaryOperatorInternal {
    Equal(BinaryOperatorEqual),
    NotEqual(BinaryOperatorNotEqual),
    LessThan(BinaryOperatorLessThan),
    LessThanOrEqual(BinaryOperatorLessThanOrEqual),
    GreaterThan(BinaryOperatorGreaterThan),
    GreaterThanOrEqual(BinaryOperatorGreaterThanOrEqual),
}

/// Binary operators
///
/// This enum represents all the binary operators.
///
/// # Variants
///
/// * `Equal` - The `==` operator
/// * `NotEqual` - The `!=` operator
/// * `LessThan` - The `<` operator
/// * `LessThanOrEqual` - The `<=` operator
/// * `GreaterThan` - The `>` operator
/// * `GreaterThanOrEqual` - The `>=` operator
pub enum BinaryOperator {
    /// The `==` operator
    Equal,
    /// The `!=` operator
    NotEqual,
    /// The `<` operator
    LessThan,
    /// The `<=` operator
    LessThanOrEqual,
    /// The `>` operator
    GreaterThan,
    /// The `>=` operator
    GreaterThanOrEqual,
}

impl From<BinaryOperatorInternal> for BinaryOperator {
    fn from(value: BinaryOperatorInternal) -> Self {
        match value {
            BinaryOperatorInternal::Equal(_) => BinaryOperator::Equal,
            BinaryOperatorInternal::NotEqual(_) => BinaryOperator::NotEqual,
            BinaryOperatorInternal::LessThan(_) => BinaryOperator::LessThan,
            BinaryOperatorInternal::LessThanOrEqual(_) => BinaryOperator::LessThanOrEqual,
            BinaryOperatorInternal::GreaterThan(_) => BinaryOperator::GreaterThan,
            BinaryOperatorInternal::GreaterThanOrEqual(_) => BinaryOperator::GreaterThanOrEqual,
        }
    }
}

struct BinaryOperatorEqual;

impl<'a> Visitor<'a, u8> for BinaryOperatorEqual {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        recognize(Token::Equal, scanner)?;
        recognize(Token::Equal, scanner)?;
        Ok(BinaryOperatorEqual)
    }
}

struct BinaryOperatorNotEqual;

impl<'a> Visitor<'a, u8> for BinaryOperatorNotEqual {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        recognize(Token::Exclamation, scanner)?;
        recognize(Token::Equal, scanner)?;
        Ok(BinaryOperatorNotEqual)
    }
}

struct BinaryOperatorLessThan;

impl<'a> Visitor<'a, u8> for BinaryOperatorLessThan {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        recognize(Token::LessThan, scanner)?;
        Ok(BinaryOperatorLessThan)
    }
}

struct BinaryOperatorLessThanOrEqual;

impl<'a> Visitor<'a, u8> for BinaryOperatorLessThanOrEqual {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        recognize(Token::LessThan, scanner)?;
        recognize(Token::Equal, scanner)?;
        Ok(BinaryOperatorLessThanOrEqual)
    }
}

struct BinaryOperatorGreaterThan;

impl<'a> Visitor<'a, u8> for BinaryOperatorGreaterThan {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        recognize(Token::GreaterThan, scanner)?;
        Ok(BinaryOperatorGreaterThan)
    }
}

struct BinaryOperatorGreaterThanOrEqual;

impl<'a> Visitor<'a, u8> for BinaryOperatorGreaterThanOrEqual {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        recognize(Token::GreaterThan, scanner)?;
        recognize(Token::Equal, scanner)?;
        Ok(BinaryOperatorGreaterThanOrEqual)
    }
}

impl<'a> Visitor<'a, u8> for BinaryOperator {
    /// Try to accept the binary operator and return the result of the visit.
    ///
    /// # Arguments
    ///
    /// * `scanner` - The scanner to accept the binary operator for.
    ///
    /// # Returns
    ///
    /// The result of the visit.
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        let acceptor = Acceptor::new(scanner)
            .try_or(BinaryOperatorInternal::Equal)?
            .try_or(BinaryOperatorInternal::NotEqual)?
            .try_or(BinaryOperatorInternal::LessThan)?
            .try_or(BinaryOperatorInternal::LessThanOrEqual)?
            .try_or(BinaryOperatorInternal::GreaterThan)?
            .try_or(BinaryOperatorInternal::GreaterThanOrEqual)?
            .finish()
            .ok_or(ParseError::UnexpectedToken)?;
        Ok(acceptor.into())
    }
}
