use noa_parser::acceptor::Acceptor;
use noa_parser::bytes::components::groups::GroupKind;
use noa_parser::bytes::matchers::match_pattern;
use noa_parser::bytes::primitives::number::Number;
use noa_parser::bytes::primitives::whitespace::OptionalWhitespaces;
use noa_parser::errors::{ParseError, ParseResult};
use noa_parser::matcher::{Match, MatchSize};
use noa_parser::peek::peek;
use noa_parser::recognizer::{Recognizable, Recognizer};
use noa_parser::scanner::Scanner;
use noa_parser::visitor::Visitor;

// ------------------------------------------------------------
// ExpressionInternal
// ------------------------------------------------------------

#[allow(dead_code)]
#[derive(Debug)]
enum ExpressionInternal {
    Reducted(Reducted),
    RightExpression(RightExpression),
}

// ------------------------------------------------------------

#[derive(Debug)]
struct Reducted {
    lhs: usize,
    op: BinaryOperator,
    rhs: usize,
}

impl<'a> Visitor<'a, u8> for Reducted {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        OptionalWhitespaces::accept(scanner)?;
        let lhs = Number::accept(scanner)?.0;
        OptionalWhitespaces::accept(scanner)?;
        let op = Recognizer::<u8, BinaryOperator>::new(scanner)
            .try_or(BinaryOperator::Add)?
            .try_or(BinaryOperator::Mul)?
            .finish()
            .ok_or(ParseError::UnexpectedToken)?;
        OptionalWhitespaces::accept(scanner)?;
        let rhs = Number::accept(scanner)?.0;
        OptionalWhitespaces::accept(scanner)?;
        Ok(Reducted { lhs, op, rhs })
    }
}

// ------------------------------------------------------------
//  +++ RightExpression
// ------------------------------------------------------------

#[derive(Debug)]
struct RightExpression {
    lhs: usize,
    op: BinaryOperator,
    rhs: Box<Expression>,
}

impl<'a> Visitor<'a, u8> for RightExpression {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        OptionalWhitespaces::accept(scanner)?;
        let lhs = Number::accept(scanner)?.0;
        OptionalWhitespaces::accept(scanner)?;
        let op = Recognizer::<u8, BinaryOperator>::new(scanner)
            .try_or(BinaryOperator::Add)?
            .try_or(BinaryOperator::Mul)?
            .finish()
            .ok_or(ParseError::UnexpectedToken)?;
        OptionalWhitespaces::accept(scanner)?;
        let rhs = Expression::accept(scanner)?;
        OptionalWhitespaces::accept(scanner)?;
        Ok(RightExpression {
            lhs,
            op,
            rhs: Box::new(rhs),
        })
    }
}

// ------------------------------------------------------------
// BinaryOperator
// ------------------------------------------------------------

#[derive(Debug)]
enum BinaryOperator {
    Add,
    Mul,
}

impl Match<u8> for BinaryOperator {
    fn matcher(&self, data: &[u8]) -> (bool, usize) {
        match self {
            BinaryOperator::Add => match_pattern(b"+", data),
            BinaryOperator::Mul => match_pattern(b"*", data),
        }
    }
}

impl MatchSize for BinaryOperator {
    fn size(&self) -> usize {
        match self {
            BinaryOperator::Add => 1,
            BinaryOperator::Mul => 1,
        }
    }
}

impl<'a> Recognizable<'a, u8, BinaryOperator> for BinaryOperator {
    fn recognize(self, scanner: &mut Scanner<'a, u8>) -> ParseResult<Option<BinaryOperator>> {
        if scanner.is_empty() {
            return Ok(None);
        }
        let (matched, size) = self.matcher(scanner.remaining());
        if matched {
            scanner.bump_by(size);
            return Ok(Some(self));
        }
        Ok(None)
    }
}

// ------------------------------------------------------------
// Expression
// ------------------------------------------------------------

/// Final result of the expression.
#[allow(dead_code)]
#[derive(Debug)]
enum Expression {
    /// Both lhs and rhs are reduced.
    Reduced {
        lhs: usize,
        op: BinaryOperator,
        rhs: usize,
    },
    /// Only lhs is reduced.
    RightExpression {
        lhs: usize,
        op: BinaryOperator,
        rhs: Box<Expression>,
    },
}

impl From<ExpressionInternal> for Expression {
    fn from(value: ExpressionInternal) -> Self {
        match value {
            ExpressionInternal::Reducted(reduced) => Expression::Reduced {
                lhs: reduced.lhs,
                op: reduced.op,
                rhs: reduced.rhs,
            },
            ExpressionInternal::RightExpression(right) => Expression::RightExpression {
                lhs: right.lhs,
                op: right.op,
                rhs: right.rhs,
            },
        }
    }
}

impl<'a> Visitor<'a, u8> for Expression {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        OptionalWhitespaces::accept(scanner)?;
        // Check if there is a parenthesis
        let result = peek(GroupKind::Parenthesis, scanner)?;

        match result {
            Some(peeked) => {
                // Parse the inner expression
                let mut inner_scanner = Scanner::new(peeked.peeked_slice());
                let inner_result = Expression::accept(&mut inner_scanner)?;
                scanner.bump_by(peeked.end_slice);
                Ok(inner_result)
            }
            None => {
                // Parse the reduced expression or the right expression
                let accepted = Acceptor::new(scanner)
                    .try_or(ExpressionInternal::RightExpression)?
                    .try_or(ExpressionInternal::Reducted)?
                    .finish()
                    .ok_or(ParseError::UnexpectedToken)?;

                Ok(accepted.into())
            }
        }
    }
}

fn main() {
    let data = b"1 + 2";
    let mut scanner = Scanner::new(data);
    let result = Expression::accept(&mut scanner);
    println!("{:?}", result); // Ok(Reduced { lhs: 1, op: Add, rhs: 2 })

    let data = b"1 + (2 * 3)";
    let mut scanner = Scanner::new(data);
    let result = Expression::accept(&mut scanner);
    println!("{:?}", result); // Ok(RightExpression { lhs: 1, op: Add, rhs: Reduced { lhs: 2, op: Mul, rhs: 3 } })

    let data = b"1 + (2 * 3 * ( 7 + 8))";
    let mut scanner = Scanner::new(data);
    let result = Expression::accept(&mut scanner);
    println!("{:?}", result); //Ok(RightExpression { lhs: 1, op: Add, rhs: RightExpression { lhs: 2, op: Mul, rhs: RightExpression { lhs: 3, op: Mul, rhs: Reduced { lhs: 7, op: Add, rhs: 8 } } } })

    let data = b"1 + 2 + 3";
    let mut scanner = Scanner::new(data);
    let result = Expression::accept(&mut scanner);
    println!("{:?}", result); // Ok(RightExpression { lhs: 1, op: Add, rhs: Reduced { lhs: 2, op: Add, rhs: 3 } })
}
