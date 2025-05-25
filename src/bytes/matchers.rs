/// Attempt to match a single character against a byte slice.
///
/// # Arguments
///
/// * `pattern` - The character to match against.
/// * `data` - The byte slice to match against.
///
/// # Returns
///
/// A tuple containing a boolean indicating whether the match succeeded and
/// the number of bytes consumed if the match succeeded.
pub fn match_char(pattern: char, data: &[u8]) -> (bool, usize) {
    (pattern as u8 == data[0], 1)
}

/// Attempt to match a byte slice against a byte slice.
///
/// # Arguments
///
/// * `pattern` - The byte slice to match against.
/// * `data` - The byte slice to match against.
///
/// # Returns
///
/// A tuple containing a boolean indicating whether the match succeeded and
/// the number of bytes consumed if the match succeeded.
pub fn match_pattern(pattern: &[u8], data: &[u8]) -> (bool, usize) {
    if pattern.is_empty() {
        return (false, 0);
    }

    if pattern.len() > data.len() {
        return (false, 0);
    }

    if pattern.eq_ignore_ascii_case(&data[..pattern.len()]) {
        return (true, pattern.len());
    }

    (false, 0)
}

/// Attempt to match a number against a byte slice.
///
/// # Arguments
///
/// * `data` - The byte slice to match against.
///
/// # Returns
///
/// A tuple containing a boolean indicating whether the match succeeded and
/// the number of bytes consumed if the match succeeded.
pub fn match_number(data: &[u8]) -> (bool, usize) {
    if data.is_empty() {
        return (false, 0);
    }

    let mut pos = 0;
    let mut found = false;

    loop {
        if pos == data.len() {
            break;
        }
        if data[pos].is_ascii_digit() {
            pos += 1;
            found = true;
            continue;
        }
        break;
    }

    (found, pos)
}

/// Attempt to match a string against a byte slice.
/// Stop matching when a punctuation character is encountered.
///  * U+0021 ..= U+002F ! " # $ % & ' ( ) * + , - . /, or
///  * U+003A ..= U+0040 : ; < = > ? @, or
///  * U+005B ..= U+0060 [ \ ] ^ _ `, or
///  * U+007B ..= U+007E { | } ~
///
/// # Arguments
///
/// * `data` - The byte slice to match against.
///
/// # Returns
///
/// A tuple containing a boolean indicating whether the match succeeded and
/// the number of bytes consumed if the match succeeded.
pub fn match_string(data: &[u8]) -> (bool, usize) {
    if data.is_empty() {
        return (false, 0);
    }

    let mut pos = 0;
    let mut found = false;

    loop {
        if pos == data.len() {
            break;
        }
        if !data[pos].is_ascii_punctuation() {
            pos += 1;
            found = true;
            continue;
        }
        break;
    }

    (found, pos)
}

#[cfg(test)]
mod tests {
    use crate::bytes::matchers::{match_char, match_number, match_pattern, match_string};

    #[test]
    fn test_match_char() {
        let (result, consumed) = match_char('a', b"abc");
        assert!(result);
        assert_eq!(consumed, 1);

        let (result, consumed) = match_char('b', b"abc");
        assert!(!result);
        assert_eq!(consumed, 1);
    }

    #[test]
    fn test_match_pattern() {
        let (result, consumed) = match_pattern(b"abc", b"abcdef");
        assert!(result);
        assert_eq!(consumed, 3);

        let (result, consumed) = match_pattern(b"abc", b"bbcdefg");
        assert!(!result);
        assert_eq!(consumed, 0);
    }

    #[test]
    fn test_match_number() {
        let (result, consumed) = match_number(b"123abc");
        assert!(result);
        assert_eq!(consumed, 3);

        let (result, consumed) = match_number(b"abc123");
        assert!(!result);
        assert_eq!(consumed, 0);
    }

    #[test]
    fn test_match_string() {
        let (result, consumed) = match_string(b"abc123(");
        assert!(result);
        assert_eq!(consumed, 6);
    }
}
