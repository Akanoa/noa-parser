# Noa parser

Is a general purpose framework parser allowing to parser any type of data without allocation.

It provides some primitives. But can be extended with custom definitions.

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
    let data = "hello world";
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
    let data = "::<>b".chars().collect::<[char; 4]>();
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

As soon an object implements `Match` and `MatchSize`, it also implements `Recognizable` and can be used to recognize a number.

```rust
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

`Recognizable` is a trait that allows you to recognize a pattern. But most of the time you want to recognize a succession of patterns.

Like the `Recognizable` trait, `Visitor` takes the scanner as an argument and tries to determine wether the pattern is present or not.

But, unlike `Recognizable`, you can call a `Visitor` inside another `Visitor` to detect more complex patterns.

For example "::<45>", the data wanted are the number "45", but embedded in the turbofish operator.

Because recognizing numbers is a common operation, the framework provides a builtin `Number` object which implements `Visitor` to recognize a number.

So to recognize a turbofish value, you have to recognize the start of the turbofish operator "::<", then the number, and then the end of the turbofish operator ">".

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
#[derive(Debug)]
struct Turbofish(usize);

struct TurbofishStartTokens;

// Implement the `Visitor` trait for the turbofish operator start tokens.
impl <'a> Visitor<'a, u8> for TurbofishStartTokens {
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