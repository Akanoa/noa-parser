use noa_parser::acceptor::Acceptor;
use noa_parser::bytes::primitives::number::Number;
use noa_parser::bytes::primitives::string::DataString;
use noa_parser::bytes::token::Token;
use noa_parser::errors::ParseError::UnexpectedToken;
use noa_parser::errors::ParseResult;
use noa_parser::recognizer::recognize;
use noa_parser::scanner::Scanner;
use noa_parser::visitor::Visitor;

#[derive(Debug)]
struct RgbColor(u8, u8, u8);
#[derive(Debug)]
struct HexColor(u8, u8, u8);
struct TupleColor(u8, u8, u8);

enum ColorInternal {
    Rgb(RgbColor),
    Hex(HexColor),
    Tuple(TupleColor),
}

impl From<ColorInternal> for Color {
    fn from(value: ColorInternal) -> Self {
        match value {
            ColorInternal::Rgb(rgb) => Color(rgb.0, rgb.1, rgb.2),
            ColorInternal::Hex(hex) => Color(hex.0, hex.1, hex.2),
            ColorInternal::Tuple(tuple) => Color(tuple.0, tuple.1, tuple.2),
        }
    }
}

impl<'a> Visitor<'a, u8> for TupleColor {
    fn accept(scanner: &mut Scanner<u8>) -> ParseResult<Self> {
        // recognize the rgb color start "("
        recognize(Token::OpenParen, scanner)?;
        // recognize the red number
        let red = Number::accept(scanner)?.0;
        // recognize the comma
        recognize(Token::Comma, scanner)?;
        recognize(Token::Whitespace, scanner)?;
        // recognize the green number
        let green = Number::accept(scanner)?.0;
        // recognize the comma
        recognize(Token::Comma, scanner)?;
        recognize(Token::Whitespace, scanner)?;
        // recognize the blue number
        let blue = Number::accept(scanner)?.0;
        // recognize the rgb color end ")"
        recognize(Token::CloseParen, scanner)?;
        Ok(TupleColor(red, green, blue))
    }
}

impl<'a> Visitor<'a, u8> for RgbColor {
    fn accept(scanner: &mut Scanner<u8>) -> ParseResult<Self> {
        let prefix = DataString::<&str>::accept(scanner)?.0;

        if prefix != "rgb" {
            return Err(UnexpectedToken);
        }

        // recognize the rgb color start "("
        recognize(Token::OpenParen, scanner)?;
        // recognize the red number
        let red = Number::accept(scanner)?.0;
        // recognize the comma
        recognize(Token::Comma, scanner)?;
        recognize(Token::Whitespace, scanner)?;
        // recognize the green number
        let green = Number::accept(scanner)?.0;
        // recognize the comma
        recognize(Token::Comma, scanner)?;
        recognize(Token::Whitespace, scanner)?;
        // recognize the blue number
        let blue = Number::accept(scanner)?.0;
        // recognize the rgb color end ")"
        recognize(Token::CloseParen, scanner)?;
        Ok(RgbColor(red, green, blue))
    }
}

impl<'a> Visitor<'a, u8> for HexColor {
    fn accept(scanner: &mut Scanner<u8>) -> ParseResult<Self> {
        recognize(Token::Sharp, scanner)?;
        let content = DataString::<&str>::accept(scanner)?.0;
        let (red, green, blue) = (
            u8::from_str_radix(&content[0..2], 16)?,
            u8::from_str_radix(&content[2..4], 16)?,
            u8::from_str_radix(&content[4..6], 16)?,
        );
        Ok(HexColor(red, green, blue))
    }
}

#[derive(Debug)]
pub struct Color(u8, u8, u8);

impl<'a> Visitor<'a, u8> for Color {
    fn accept(scanner: &mut Scanner<u8>) -> ParseResult<Self> {
        let color = Acceptor::new(scanner)
            .try_or(ColorInternal::Hex)?
            .try_or(ColorInternal::Rgb)?
            .try_or(ColorInternal::Tuple)?
            .finish()
            .ok_or(UnexpectedToken)?;
        Ok(color.into())
    }
}

fn main() {
    let data = b"rgb(255, 0, 0)";
    let mut scanner = Scanner::new(data);
    let result = Color::accept(&mut scanner);
    println!("{:?}", result);

    let data = b"#ff0000";
    let mut scanner = Scanner::new(data);
    let result = Color::accept(&mut scanner);
    println!("{:?}", result);

    let data = b"(255, 0, 0)";
    let mut scanner = Scanner::new(data);
    let result = Color::accept(&mut scanner);
    println!("{:?}", result);
}
