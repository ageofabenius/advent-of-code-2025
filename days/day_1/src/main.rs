const INPUT: &str = include_str!("../input.txt");

/// Count the lines, this is needed for the const definition of `INPUT_LINES` as
/// a fixed-size array
const NUM_LINES: usize = count_lines(INPUT);

const fn count_lines(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut count = 0;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            count += 1;
        }
        i += 1;
    }

    count
}

/// Define `INPUT_LINES` as a fixed-size array, and split the input into it
const INPUT_LINES: [&str; NUM_LINES] = split_lines(INPUT, '\n');

const fn split_lines(s: &str, d: char) -> [&str; NUM_LINES] {
    // Shadow d as a u8, this function works with bytes rather than characters
    // as &str can't be indexed directly
    let d = d as u8;
    let bytes = s.as_bytes();

    // Initialize `lines` array to empty &'static str
    let mut lines: [&str; NUM_LINES] = [""; NUM_LINES];

    let mut i = 0;
    let mut line_num = 0;
    let mut line_start = 0;
    while i < s.len() {
        if bytes[i] == d {
            // The following line does not work, because:
            // cannot call conditionally-const operator in constant functions
            //
            // lines[line_num] = &s[line_start..i];

            // So we instead construct a &str manually with a raw pointer
            unsafe {
                let ptr_line_start: *const u8 = s.as_ptr().add(line_start);
                let line_length = i - line_start;
                let line_slice: &str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                    ptr_line_start,
                    line_length,
                ));

                lines[line_num] = line_slice;
            }

            // Increment to the next line
            line_num += 1;
            // Define the start of the next line as the next index
            line_start = i + 1;

            // Skip over the next byte as it's the new line_start
            i += 2;
        } else {
            // Move to the next byte
            i += 1;
        }
    }

    lines
}

const fn parse_rotation(s: &str) -> i32 {
    let bytes = s.as_bytes();
    let sign = match bytes[0] {
        b'L' => -1,
        b'R' => 1,
        _ => unreachable!(),
    };
    let number: i32 = parse_i32(bytes, Some(1));

    sign * number
}

const fn parse_i32(bytes: &[u8], offset: Option<usize>) -> i32 {
    let mut number: i32 = 0;
    let mut i: usize = if let Some(i) = offset { i } else { 0 };
    while i < bytes.len() {
        let byte: u8 = bytes[i];

        // Since this is ASCII, we can take the byte character's value and slide
        // it over by the value for ASCII 0
        let digit = (byte - b'0') as i32;

        number = (number * 10) + digit;

        i += 1;
    }

    number
}

const fn do_the_thing(lines: &[&str]) -> i32 {
    let mut position = 50;

    let mut count_of_zeroes = 0;

    let mut i = 0;
    while i < lines.len() {
        // Apply rotation to position
        let rotation = parse_rotation(lines[i]);
        position += rotation;

        // Adjust position to be from 0 - 99
        position = position % 100;

        if position == 0 {
            count_of_zeroes += 1;
        }

        i += 1;
    }

    count_of_zeroes
}
const ANSWER: i32 = do_the_thing(&INPUT_LINES);

fn main() {
    println!("ANSWER: {ANSWER}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_i32() {
        assert_eq!(parse_i32(b"123", None), 123);
        assert_eq!(parse_i32(b"0", None), 0);
        assert_eq!(parse_i32(b"L999", Some(1)), 999);
    }

    #[test]
    fn test_parse_rotation() {
        assert_eq!(parse_rotation("L31"), -31);
        assert_eq!(parse_rotation("R43"), 43);
    }

    #[test]
    fn test_do_the_thing() {
        assert_eq!(do_the_thing(&["L50"]), 1);
        assert_eq!(do_the_thing(&["R50"]), 1);

        assert_eq!(do_the_thing(&["R50", "R100"]), 2);

        assert_eq!(
            do_the_thing(&[
                "R27", "R13", "L8", "R30", "R22", "L9", "L32", "R22", "R20", "R16",
            ]),
            2
        );
    }
}
