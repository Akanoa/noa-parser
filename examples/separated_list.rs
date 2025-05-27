use noa_parser::bytes::primitives::number::Number;
use noa_parser::bytes::token::Token;
use noa_parser::errors::ParseResult;
use noa_parser::recognizer::recognize;
use noa_parser::scanner::Scanner;
use noa_parser::separated_list::SeparatedList;
use noa_parser::visitor::Visitor;

#[derive(Debug)]
struct Separator;

impl<'a> Visitor<'a, u8> for Separator {
    fn accept(scanner: &mut noa_parser::scanner::Scanner<u8>) -> ParseResult<Self> {
        recognize(Token::Tilde, scanner)?;
        recognize(Token::Tilde, scanner)?;
        recognize(Token::Tilde, scanner)?;
        Ok(Separator)
    }
}

fn main() {
    let data = b"1~~~2~~~3~~~4";
    let mut scanner = Scanner::new(data);
    let result =
        SeparatedList::<u8, Number<usize>, Separator>::accept(&mut scanner).map(|x| x.data);
    println!("{:?}", result); // Ok([Number(1), Number(2), Number(3), Number(4)])
}
