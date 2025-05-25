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
    let data = [':', ':', '<', '>', 'b'];
    let scanner = noa_parser::scanner::Scanner::new(&data);
    let result = Turbofish.matcher(&scanner);
    println!("{:?}", result); // ( true, 4 ) because the turbofish operator is 4 char

    let data = ['a', ':', ':', '<', '>', 'b'];
    let scanner = noa_parser::scanner::Scanner::new(&data);
    let result = Turbofish.matcher(&scanner);
    println!("{:?}", result); // ( false, 0 ) because doesn't match the turbofish operator
}
