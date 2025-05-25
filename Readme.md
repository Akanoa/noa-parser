# Noa parser

Is a general purpose framework parser allowing to parser any type of data without allocation.

It provides some primitives.

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

You want that all characters are matched.

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
    let data = "::<>b".chars().collect::<Vec<char>>();
    let mut scanner = noa_parser::scanner::Scanner::new(&data);
    let result = Turbofish.matcher(&mut scanner);
    println!("{:?}", result);
}
```

## Visitor