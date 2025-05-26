# Noa parser

Is an extensible general purpose framework parser allowing to parser any type of data without allocation.

## Scanner

The scanner is a simple wrapper a slice of data.

This data can be bytes, chars or any other type.

The scanner is the building block around which parsers are built.

It provides basic operations such as:

- bumping the cursor
- get the current position
- remaining data to be scanned
- rewinding the cursor

Parsers only use most of the operations internally.

### Usage

```rust
use noa_parser::scanner::Scanner;
fn main() {
    let data = b"hello world";
    let mut scanner = Scanner::new(data);
}
```

## Match and MatchSize

Parsing data involves recognizing a pattern in the data.

To help this recognition. The framework provides two traits:

- `Match` : defines how to recognize a pattern
- `MatchSize` : defines how to get the size of a pattern recognized

```rust
pub trait Match<T> {
    /// Returns true if the data matches the pattern.
    ///
    /// # Arguments
    /// data - the data to match
    ///
    /// # Returns
    /// (true, index) if the data matches the pattern,
    /// (false, index) otherwise
    fn matcher(&self, data: &[T]) -> (bool, usize);
}

pub trait MatchSize {
    /// Returns the size of the matchable object.
    fn size(&self) -> usize;
}
```

### Usage

For example, if you want to recognize the turbofish pattern "::<>".

You want that all characters to be matched.

To achieve, we need an object that implements `Match` and `MatchSize`.

Here the object will be the `Turbofish` struct.

```rust
use noa_parser::matcher::{Match, MatchSize};

/// Pattern to match.
const TURBOFISH: [char; 4] = [':', ':', '<', '>'];

/// Handle turbofish operator.
struct Turbofish;

/// Match turbofish operator.
impl Match<char> for Turbofish {
    fn matcher(&self, data: &[char]) -> (bool, usize) {
        let pattern = &TURBOFISH;
        if data.len() < pattern.len() {
            return (false, 0);
        }
        if &data[..pattern.len()] == pattern {
            return (true, pattern.len());
        }
        (false, 0)
    }
}

/// Return the size of the turbofish operator.
impl MatchSize for Turbofish {
    fn size(&self) -> usize {
        TURBOFISH.len()
    }
}

fn main() {
    let data = [':', ':', '<', '>'];
    let mut scanner = noa_parser::scanner::Scanner::new(&data);
    let result = Turbofish.matcher(&mut scanner);
    println!("{:?}", result);
}
```

## Recognizable

Once you have an object that implements `Match` and `MatchSize`, you can use it to recognize a pattern.

For static data it's not that useful, but for something with not defined it can be interesting.

You want to recognize a number.

You need an object able to match a sequence of digits.

Because it's a common operation, the framework provides a builtin function to do it: `match_number`.

As soon an object implements `Match` and `MatchSize`, it also implements `Recognizable` and can be used to recognize a
number.

```rust
use noa_parser::matcher::MatchSize;
use noa_parser::scanner::Scanner;
use noa_parser::errors::ParseResult;
pub trait Recognizable<'a, T, V>: MatchSize {
    /// Try to recognize the object for the given scanner.
    ///
    /// # Type Parameters
    /// V - The type of the object to recognize
    ///
    /// # Arguments
    /// * `scanner` - The scanner to recognize the object for.
    ///
    /// # Returns
    /// * `Ok(Some(V))` if the object was recognized,
    /// * `Ok(None)` if the object was not recognized,
    /// * `Err(ParseError)` if an error occurred
    ///
    fn recognize(self, scanner: &mut Scanner<'a, T>) -> ParseResult<Option<V>>;
}
```

### Usage

```rust
use noa_parser::bytes::matchers::match_number;
use noa_parser::matcher::{Match, MatchSize};
use noa_parser::recognizer::Recognizable;

struct TokenNumber;

/// Implement the `Match` trait for the token number.
impl Match<u8> for TokenNumber {
    fn matcher(&self, data: &[u8]) -> (bool, usize) {
        match_number(data)
    }
}

/// Implement the `MatchSize` trait for the token number.
impl MatchSize for TokenNumber {
    fn size(&self) -> usize {
        // The size of the token number is 0 because it's not defined
        0
    }
}

fn main() {
    let data = b"123abc";
    let mut scanner = noa_parser::scanner::Scanner::new(data);
    let result = TokenNumber.recognize(&mut scanner);
    println!("{:?}", result); // Ok(Some([49, 50, 51]))
    // If the result is successful
    if let Ok(Some(data)) = result {
        // Convert the data to a string
        let str_data = std::str::from_utf8(data).unwrap();
        // Convert the string to a number
        let result = str_data.parse::<usize>().unwrap();
        println!("{}", result); // 123
    }
}
```

## Visitor

`Recognizable` is a trait that allows you to recognize a pattern. But most of the time you want to recognize a
succession of patterns.

Like the `Recognizable` trait, `Visitor` takes the scanner as an argument and tries to determine whether the pattern is
present or not.

```rust
use noa_parser::matcher::MatchSize;
use noa_parser::scanner::Scanner;
use noa_parser::errors::ParseResult;
/// A `Visitor` is a trait that allows to define how to visit a `Scanner`.
///
/// When a `Visitor` is used on a `Scanner`, it will consume the input from the
/// scanner and return the result of the visit.
///
/// # Type Parameters
///
/// * `T` - The type of the data to visit.
///
/// # Associated Functions
///
/// * `accept` - Try to accept the `Scanner` and return the result of the visit.
pub trait Visitor<'a, T>: Sized {
    /// Try to accept the `Scanner` and return the result of the visit.
    ///
    /// # Arguments
    ///
    /// * `scanner` - The scanner to accept.
    ///
    /// # Returns
    ///
    /// The result of the visit.
    fn accept(scanner: &mut Scanner<'a, T>) -> ParseResult<Self>;
}
```

But, unlike `Recognizable`, you can call a `Visitor` inside another `Visitor` to detect more complex patterns.

For example, "::<45>", the data wanted are the number "45", but embedded in the turbofish operator.

Because recognizing numbers is a common operation, the framework provides a builtin `Number` object which implements
`Visitor` to recognize a number.

So to recognize a turbofish value, you have to recognize the start of the turbofish operator "::<", then the number, and
then the end of the turbofish operator ">".

The recognition of the number is done by calling the `accept` method of the `Number` object.

```rust
use noa_parser::bytes::primitives::number::Number;
use noa_parser::bytes::token::Token;
use noa_parser::errors::ParseResult;
use noa_parser::recognizer::recognize;
use noa_parser::visitor::Visitor;

#[derive(Debug)]
struct Turbofish(usize);

// Implement the `Visitor` trait for the turbofish operator.
impl<'a> Visitor<'a, u8> for Turbofish {
    fn accept(scanner: &mut noa_parser::scanner::Scanner<u8>) -> ParseResult<Self> {
        // recognize the turbofish operator start "::<".
        recognize(Token::Colon, scanner)?;
        recognize(Token::Colon, scanner)?;
        recognize(Token::LessThan, scanner)?;
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
```

If you want you can embed the turbofish operator start pattern inside its own `Visitor`.

```rust
use noa_parser::visitor::Visitor;
use noa_parser::scanner::Scanner;
use noa_parser::errors::ParseResult;
use noa_parser::recognizer::recognize;
use noa_parser::bytes::token::Token;
use noa_parser::bytes::primitives::number::Number;

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
```

There is no limit of embedding depth.

## Match alternatives

Sometimes your parsing path will branch between two or more paths.

You may need to recognize an operator, for example.

The `Recognizer` allows to check multiple patterns.

```rust
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

```

## Accept alternatives

When the recognizer is not enough, you need to check several visitors.

That's the purpose of the `Acceptor` object.

For example, colors can be defined in different ways.

- #ff0000
- (255, 0, 0)
- rgb(255, 0, 0)

If your parser wants to accept every pattern, you must test them successively then stop at the first matching pattern.

To achieve this, the framework provides an object called `Acceptor` which takes several `Visitor`.

Because of rust, all your results must be of the same type. So is a union as the form of an enumeration of visitable
types.

Here:

```rust,ignore
enum ColorInternal {
    Rgb(RgbColor),
    Hex(HexColor),
    Tuple(TupleColor),
}
```

Then define the visitable types:

```rust,ignore
#[derive(Debug)]
struct RgbColor(u8, u8, u8);
#[derive(Debug)]
struct HexColor(u8, u8, u8);
struct TupleColor(u8, u8, u8);
```

To implement their `Visitor`:

```rust,ignore
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
        // built-in visitor allows to recognize any string until punctuation
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
```

Then define the output `Color` type:

```rust,ignore
#[derive(Debug)]
pub struct Color(u8, u8, u8);

impl From<ColorInternal> for Color {
    fn from(value: ColorInternal) -> Self {
        match value {
            ColorInternal::Rgb(rgb) => Color(rgb.0, rgb.1, rgb.2),
            ColorInternal::Hex(hex) => Color(hex.0, hex.1, hex.2),
            ColorInternal::Tuple(tuple) => Color(tuple.0, tuple.1, tuple.2),
        }
    }
}
```

And finally define the `Color` visitor:

```rust,ignore
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
    println!("{:?}", result); // Ok(Color(255, 0, 0))

    let data = b"#ff0000";
    let mut scanner = Scanner::new(data);
    let result = Color::accept(&mut scanner);
    println!("{:?}", result); // Ok(Color(255, 0, 0))

    let data = b"(255, 0, 0)";
    let mut scanner = Scanner::new(data);
    let result = Color::accept(&mut scanner);
    println!("{:?}", result); // Ok(Color(255, 0, 0))
}
```

## Delimited groups

Sometimes parsing involves nested datastructures where parse are embedded in other parse.

If you have this expression: "1 + (2 * 3)", you first need to discover all binary groups, here

- "Num(1) + Group(2 * 3)"'
- "Num(2) * Num(3)"

To be able to resolve the whole expression, you first need to understand the concept of group between parenthesis, get
the inner expression, then parse it.

That's the purpose of the `peek` function.

It takes a `Peekable` object and try to get the substring that matches the given `Peekable`.

The framework provides two `Peekable` implementations:

- `GroupKind::Parenthesis` : A group enclosed in parentheses
- `GroupKind::Quotes` : A group enclosed in single quotes, the backslash `\'` is escaped
- `GroupKind::DoubleQuotes` : A group enclosed in double quotes, the backslash `\"` is escaped
- `Until` : A group until the given `Recognizable`
- `UntilEnd` : A group until the end of the input

```rust
use noa_parser::bytes::components::groups::GroupKind;
use noa_parser::peek::peek;

fn main() {
    let data = b"(2 * 3)";
    let mut scanner = noa_parser::scanner::Scanner::new(data);
    let result = peek(GroupKind::Parenthesis, &mut scanner).expect("failed to parse").expect("failed to peek");
    println!(
        "{}",
        String::from_utf8_lossy(result.peeked_slice()) // 2 * 3
    );
}
```

An example of the peeking usage is available in the [expression](examples/expression.rs) example.