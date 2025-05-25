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
