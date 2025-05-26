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
