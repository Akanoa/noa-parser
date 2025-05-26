//! Group components

use crate::bytes::token::Token;
use crate::errors::ParseResult;
use crate::peek::{PeekResult, Peekable};
use crate::recognizer::Recognizable;
use crate::scanner::Scanner;

/// Try to recognize either a start group or an end group token.
///
/// If the start group token is recognized, increment the balancing counter.
/// If the end group token is recognized, decrement the balancing counter.
/// If neither is recognized, move the tokenizer by one byte.
///
/// # Arguments
///
/// * `tokenizer` - The tokenizer to use
/// * `balance` - A mutable reference to the balancing counter
/// * `start` - The start group token to recognize
/// * `end` - The end group token to recognize
///
/// # Errors
///
/// Returns `Err(ParseError)` if the tokenizer encounters an error.
///
/// # Examples
///
///
pub fn match_for_balanced_group<'a, V1, T1, V2, T2>(
    tokenizer: &mut Scanner<'a, u8>,
    balance: &mut usize,
    start: T1,
    end: T2,
) -> ParseResult<()>
where
    T1: Recognizable<'a, u8, V1> + Copy,
    T2: Recognizable<'a, u8, V2> + Copy,
{
    // try to recognize start group token
    match start.recognize(tokenizer)? {
        // if not start token try to recognize end group token
        None => match end.recognize(tokenizer)? {
            // if end group token decrement balancing counter
            Some(_end_token) => *balance -= 1,
            // if neither, move by one byte
            None => {
                tokenizer.bump_by(1);
                return Ok(());
            }
        },
        // if start group token increment balancing counter
        Some(_start_token) => *balance += 1,
    };

    Ok(())
}

/// A closure that takes a slice of bytes and returns a `PeekResult` indicating
/// whether the slice matches a balanced group.
///
/// A balanced group is a sequence of bytes that has the same number of start
/// and end group tokens. The start group token is recognized by the `start`
/// parameter and the end group token is recognized by the `end` parameter.
///
/// The closure returns `Ok(PeekResult::Found { end_slice, start, end })` if the
/// slice matches a balanced group, `Ok(PeekResult::NotFound)` if the slice
/// does not match a balanced group, and `Err(ParseError)` if there is an error
/// recognizing the tokens.
///
/// # Arguments
///
/// * `start` - The start group token to recognize
/// * `end` - The end group token to recognize
///
/// # Returns
///
/// A closure that takes a slice of bytes and returns a `PeekResult` indicating
/// whether the slice matches a balanced group.
pub fn match_group<'a, V1, T1, V2, T2>(
    start: T1,
    end: T2,
) -> impl Fn(&'a [u8]) -> ParseResult<PeekResult<T1, T2>> + 'a
where
    T1: Recognizable<'a, u8, V1> + Copy + 'a,
    T2: Recognizable<'a, u8, V2> + Copy + 'a,
{
    move |input: &'a [u8]| {
        // 0 if number of start token equals number of end token
        // i.e: ( 5 + 3 - ( 10 * 8 ) ) => 2 "(" and 2 ")" => balanced
        //      ( 5 + 3 - ( 10 * 8 )   => 2 "(" and 1 ")" => unbalanced
        let mut balance = 0;

        let mut tokenizer = Scanner::new(input);

        loop {
            match_for_balanced_group(&mut tokenizer, &mut balance, start, end)?;
            // if balancing is 0 then either there is no group at all or is balanced
            if balance == 0 {
                break;
            }
        }

        // not enough bytes to create a group
        if tokenizer.current_position() == 1 {
            return Ok(PeekResult::NotFound);
        }

        Ok(PeekResult::Found {
            end_slice: tokenizer.current_position(),
            start,
            end,
        })
    }
}

/// A closure that takes a slice of bytes and returns a `PeekResult` indicating
/// whether the slice matches a delimited group.
///
/// A delimited group is a sequence of bytes that starts and ends with the same
/// token and has no other occurrence of that token in between. The token is
/// recognized by the `token` parameter and the escape token is recognized by the
/// `escape_token` parameter.
///
/// The closure returns `Ok(PeekResult::Found { end_slice, start, end })` if the
/// slice matches a delimited group, `Ok(PeekResult::NotFound)` if the slice
/// does not match a delimited group, and `Err(ParseError)` if there is an error
/// recognizing the tokens.
///
/// # Arguments
///
/// * `token` - The token to recognize at the start and end of the group
/// * `escape_token` - The escape token to recognize and ignore in the group
///
/// # Returns
///
/// A closure that takes a slice of bytes and returns a `PeekResult` indicating
/// whether the slice matches a delimited group.
pub fn match_for_delimited_group<'a, V, T, V2, T2>(
    token: T,
    escape_token: T2,
) -> impl Fn(&'a [u8]) -> ParseResult<PeekResult<T, T>> + 'a
where
    T: Recognizable<'a, u8, V> + Copy + 'a,
    T2: Recognizable<'a, u8, V2> + Copy + 'a,
{
    move |input: &'a [u8]| {
        // le groupe doit au moins faire 2 tokens de taille
        if input.len() < token.size() * 2 {
            return Ok(PeekResult::NotFound);
        }

        // on créé un scanner à partir des données
        let mut tokenizer = Scanner::new(input);

        // le groupe doit obligatoirement débuter par le token
        if token.recognize(&mut tokenizer)?.is_none() {
            return Ok(PeekResult::NotFound);
        }
        // on avance de la taille du token reconnu
        tokenizer.bump_by(token.size());

        // ce flag permet de savoir si la prédiction a été un succès
        let mut found = false;

        // tant que la slice contient des bytes, on essaie de reconnaître le token
        while !tokenizer.remaining().is_empty() {
            // si le token est reconnu quelque part dans la slice
            if token.recognize(&mut tokenizer)?.is_some() {
                // on créé un nouveau scanner qui est un token et un \ en arrière
                let mut rewind_tokenizer = Scanner::new(
                    &tokenizer.data()
                        [tokenizer.current_position() - token.size() - escape_token.size()..],
                );
                // on tente de reconnaître le \
                if escape_token.recognize(&mut rewind_tokenizer)?.is_some() {
                    // s'il est présent, le token est échappé
                    continue;
                }
                // sinon on a atteint la fin du groupe et la prédiction est un succès
                found = true;
                break;
            }
            // sinon on avance d'un byte
            tokenizer.bump_by(1);
        }

        // Si la prédiction est un échec
        if !found {
            return Ok(PeekResult::NotFound);
        }

        Ok(PeekResult::Found {
            end_slice: tokenizer.current_position(),
            start: token,
            end: token,
        })
    }
}

/// Types of groups
///
/// This enum is used to specify the type of a group in a matcher.
pub enum GroupKind {
    /// A group enclosed in parentheses
    Parenthesis,
    /// A group enclosed in single quotes
    Quotes,
    /// A group enclosed in double quotes
    DoubleQuotes,
}

impl GroupKind {
    fn matcher<'a>(&self) -> Box<dyn Fn(&'a [u8]) -> ParseResult<PeekResult<Token, Token>> + 'a>
where {
        match self {
            GroupKind::Parenthesis => Box::new(match_group(Token::OpenParen, Token::CloseParen)),
            GroupKind::Quotes => {
                Box::new(match_for_delimited_group(Token::Quote, Token::Backslash))
            }
            GroupKind::DoubleQuotes => Box::new(match_for_delimited_group(
                Token::DoubleQuote,
                Token::Backslash,
            )),
        }
    }
}

impl<'a> Peekable<'a, u8, Token, Token> for GroupKind {
    fn peek(&self, data: &Scanner<'a, u8>) -> ParseResult<PeekResult<Token, Token>> {
        self.matcher()(data.remaining())
    }
}

#[cfg(test)]
mod tests {
    use crate::bytes::components::groups::{GroupKind, match_for_delimited_group, match_group};
    use crate::bytes::token::Token;
    use crate::peek::{PeekResult, Peeking, peek};
    use crate::scanner::Scanner;

    #[test]
    fn test_match_group() {
        let data = b"( 5 + 3 - ( 10 * 8 ) ) + 54";
        let result =
            match_group(Token::OpenParen, Token::CloseParen)(data).expect("failed to parse");
        assert_eq!(
            result,
            PeekResult::Found {
                end_slice: 22,
                start: Token::OpenParen,
                end: Token::CloseParen
            }
        );
        assert_eq!(&data[..22], b"( 5 + 3 - ( 10 * 8 ) )");
    }

    #[test]
    fn test_match_group_delimited() {
        let data = b"( 5 + 3 - ( 10 * 8 ) ) + 54";
        let mut tokenizer = Scanner::new(data);
        let result = peek(GroupKind::Parenthesis, &mut tokenizer).expect("failed to parse");
        assert_eq!(
            result,
            Some(Peeking {
                start: Token::OpenParen,
                end: Token::CloseParen,
                data: &data[0..22],
                end_slice: 22
            })
        );
        assert_eq!(&data[..22], b"( 5 + 3 - ( 10 * 8 ) )");
    }

    #[test]
    fn test_match_quotes() {
        let data = b"'hello world' data";
        let result = match_for_delimited_group(Token::Quote, Token::Backslash)(data)
            .expect("failed to parse");
        assert_eq!(
            result,
            PeekResult::Found {
                end_slice: 13,
                start: Token::Quote,
                end: Token::Quote
            }
        );
        assert_eq!(&data[..13], b"'hello world'");

        let data = r#"'hello world l\'éléphant' data"#;
        let result = match_for_delimited_group(Token::Quote, Token::Backslash)(data.as_bytes())
            .expect("failed to parse");
        assert_eq!(
            result,
            PeekResult::Found {
                end_slice: 27,
                start: Token::Quote,
                end: Token::Quote
            }
        );
        assert_eq!(&data[..27], r#"'hello world l\'éléphant'"#);

        let data = "\"hello world\" data";
        let result =
            match_for_delimited_group(Token::DoubleQuote, Token::Backslash)(data.as_bytes())
                .expect("failed to parse");
        assert_eq!(
            result,
            PeekResult::Found {
                end_slice: 13,
                start: Token::DoubleQuote,
                end: Token::DoubleQuote
            }
        );
        assert_eq!(&data[..13], "\"hello world\"");

        let data = r#""hello world" data"#;
        let result =
            match_for_delimited_group(Token::DoubleQuote, Token::Backslash)(data.as_bytes())
                .expect("failed to parse");
        assert_eq!(
            result,
            PeekResult::Found {
                end_slice: 13,
                start: Token::DoubleQuote,
                end: Token::DoubleQuote
            }
        );
        assert_eq!(&data[..13], r#""hello world""#);
    }
}
