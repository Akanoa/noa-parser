use noa_parser::bytes::matchers::match_pattern;
use noa_parser::errors::{ParseError, ParseResult};
use noa_parser::matcher::{Match, MatchSize};
use noa_parser::recognizer::Recognizer;
use noa_parser::scanner::Scanner;

enum OperatorTokens {
    /// The `==` operator.
    Equal,
    /// The `!=` operator.
    NotEqual,
}

impl Match<u8> for OperatorTokens {
    fn matcher(&self, data: &[u8]) -> (bool, usize) {
        match self {
            OperatorTokens::Equal => match_pattern(b"==", data),
            OperatorTokens::NotEqual => match_pattern(b"!=", data),
        }
    }
}

impl MatchSize for OperatorTokens {
    fn size(&self) -> usize {
        match self {
            OperatorTokens::Equal => 2,
            OperatorTokens::NotEqual => 2,
        }
    }
}

fn main() -> ParseResult<()> {
    let data = b"== 2";
    let mut scanner = Scanner::new(data);
    let recognized = Recognizer::new(&mut scanner)
        .try_or(OperatorTokens::NotEqual)?
        .try_or(OperatorTokens::Equal)?
        .finish()
        .ok_or(ParseError::UnexpectedToken)?;

    println!("{}", String::from_utf8_lossy(recognized)); // ==

    let data = b"!= 2";
    let mut scanner = Scanner::new(data);
    let recognized = Recognizer::new(&mut scanner)
        .try_or(OperatorTokens::NotEqual)?
        .try_or(OperatorTokens::Equal)?
        .finish()
        .ok_or(ParseError::UnexpectedToken)?;

    println!("{}", String::from_utf8_lossy(recognized)); // !=

    let data = b"> 2";
    let mut scanner = Scanner::new(data);
    let recognized = Recognizer::new(&mut scanner)
        .try_or(OperatorTokens::NotEqual)?
        .try_or(OperatorTokens::Equal)?
        .finish()
        .ok_or(ParseError::UnexpectedToken);

    println!("{:?}", recognized); // error (UnexpectedToken)

    Ok(())
}
