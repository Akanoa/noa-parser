//! Error types
/// The result of a parse operation
pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Unexpected token have been encountered")]
    UnexpectedToken,
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}
