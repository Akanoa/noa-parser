use noa_parser::bytes::primitives::number::Number;
use noa_parser::bytes::token::Token;
use noa_parser::errors::ParseResult;
use noa_parser::recognizer::recognize;
use noa_parser::scanner::Scanner;
use noa_parser::visitor::Visitor;

#[derive(Debug)]
struct Turbofish(usize);

struct TurbofishStartTokens;

// Implement the `Visitor` trait for the turbofish operator start tokens.
impl<'a> Visitor<'a, u8> for TurbofishStartTokens {
    fn accept(scanner: &mut Scanner<'a, u8>) -> ParseResult<Self> {
        // recognize the turbofish operator start "::<".
        recognize(Token::Colon, scanner)?;
        recognize(Token::Colon, scanner)?;
        recognize(Token::LessThan, scanner)?;
        Ok(TurbofishStartTokens)
    }
}

// Implement the `Visitor` trait for the turbofish operator.
impl<'a> Visitor<'a, u8> for Turbofish {
    fn accept(scanner: &mut noa_parser::scanner::Scanner<u8>) -> ParseResult<Self> {
        // recognize the turbofish operator start "::<".
        TurbofishStartTokens::accept(scanner)?;
        // recognize the number
        let number = Number::accept(scanner)?.0;
        // recognize the turbofish operator end ">"
        recognize(Token::GreaterThan, scanner)?;
        Ok(Turbofish(number))
    }
}

fn main() {
    let data = b"::<45>garbage";
    let mut scanner = noa_parser::scanner::Scanner::new(data);
    let result = Turbofish::accept(&mut scanner);
    println!("{:?}", result); // Ok(Turbofish(45))
}
