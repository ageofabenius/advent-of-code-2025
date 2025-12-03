pub const fn parse_usize(bytes: &[u8], offset: Option<usize>, length: Option<usize>) -> usize {
    let start_index: usize = if let Some(i) = offset { i } else { 0 };
    let end_index: usize = if let Some(l) = length {
        l + start_index
    } else {
        bytes.len()
    };

    let mut number: usize = 0;
    let mut i: usize = start_index;
    while i < end_index {
        let byte: u8 = bytes[i];

        // Since this is ASCII, we can take the byte character's value and slide
        // it over by the value for ASCII 0
        let digit = (byte - b'0') as usize;

        number = (number * 10) + digit;

        i += 1;
    }

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_usize() {
        assert_eq!(parse_usize(b"12345", None, Some(5)), 12345);
        assert_eq!(parse_usize(b"12345", None, None), 12345);
        assert_eq!(parse_usize(b"R6789", Some(1), Some(4)), 6789);
        assert_eq!(parse_usize(b"R6789", Some(1), None), 6789);
    }
}
