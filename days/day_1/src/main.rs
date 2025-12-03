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

const fn parse_rotation(s: &str) -> isize {
    let bytes = s.as_bytes();
    let sign = match bytes[0] {
        b'L' => -1,
        b'R' => 1,
        _ => unreachable!(),
    };
    let number: isize = parse_isize(bytes, Some(1));

    sign * number
}

const fn parse_isize(bytes: &[u8], offset: Option<usize>) -> isize {
    let mut number: isize = 0;
    let mut i: usize = if let Some(i) = offset { i } else { 0 };
    while i < bytes.len() {
        let byte: u8 = bytes[i];

        // Since this is ASCII, we can take the byte character's value and slide
        // it over by the value for ASCII 0
        let digit = (byte - b'0') as isize;

        number = (number * 10) + digit;

        i += 1;
    }

    number
}

const fn do_the_thing(lines: &[&str]) -> isize {
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
const ANSWER: isize = do_the_thing(&INPUT_LINES);

fn do_the_second_thing(lines: &[&str]) -> isize {
    let mut position = 50;

    let mut count_of_zeroes = 0;

    let mut i = 0;
    while i < lines.len() {
        let rotation = parse_rotation(lines[i]);
        let (new_position, times_crossed_zero, times_stopped_at_zero) =
            apply_rotation(position, rotation);
        position = new_position;

        count_of_zeroes += times_crossed_zero + times_stopped_at_zero;

        i += 1;
    }

    count_of_zeroes
}

/// -> (ending_position, times_crossed_zero, times_stopped_at_zero)
/// Here, 99 -> 100 (rolling to 0) is considered times_stopped_at_zero, not times_crossed_zero
fn apply_rotation(starting_position: isize, rotation: isize) -> (isize, isize, isize) {
    // Determine the number of full rotations
    let full_rotations = dbg!((rotation / 100).abs());

    // We will apply the remaining rotation
    let remaining_rotation = dbg!(rotation % 100);

    if remaining_rotation == 0 {
        // We don't need to apply rotations, but we do need to account for
        // ending on 0 as we return that separately
        if starting_position == 0 {
            return (starting_position, full_rotations - 1, 1);
        } else {
            return (starting_position, full_rotations, 0);
        }
    }

    let position_with_wrapping = dbg!(starting_position + remaining_rotation);

    // assert!(position_with_wrapping < 199 && position_with_wrapping > -100);

    let times_crossed_zero = dbg!(if position_with_wrapping > 100 {
        1
    } else if (position_with_wrapping < 0) && (starting_position != 0) {
        1
    } else {
        0
    });

    // Adjust position to be from -99 to 99
    let mut adjusted_position = position_with_wrapping % 100;
    // Adjust position to be from 0 to 99
    if adjusted_position < 0 {
        adjusted_position += 100;
    }

    let times_stopped_at_zero = dbg!(if adjusted_position == 0 { 1 } else { 0 });

    (
        adjusted_position,
        full_rotations + times_crossed_zero,
        times_stopped_at_zero,
    )
}

// const SECOND_ANSWER: isize = do_the_second_thing(&INPUT_LINES);

fn main() {
    // println!("ANSWER: {ANSWER}");
    // println!("SECOND_ANSWER: {SECOND_ANSWER}");
    println!("SECOND_ANSWER: {}", do_the_second_thing(&INPUT_LINES));
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_parse_isize() {
        assert_eq!(parse_isize(b"123", None), 123);
        assert_eq!(parse_isize(b"0", None), 0);
        assert_eq!(parse_isize(b"L999", Some(1)), 999);
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
    }

    #[test]
    fn test_do_the_second_thing() {
        assert_eq!(do_the_second_thing(&["R50"]), 1);
        assert_eq!(do_the_second_thing(&["R50", "R50"]), 1);
        assert_eq!(do_the_second_thing(&["R50", "R50", "R50"]), 2);

        assert_eq!(do_the_second_thing(&["L50"]), 1);
        assert_eq!(do_the_second_thing(&["L50", "L50"]), 1);
        assert_eq!(do_the_second_thing(&["L50", "L50", "L50"]), 2);
    }

    #[rstest]
    #[case((50, 49), (99, 0, 0))]
    #[case((50, -49), (1, 0, 0))]
    #[case((50, 50), (0, 0, 1))]
    #[case((50, -50), (0, 0, 1))]
    #[case((50, 51), (1, 1, 0))]
    #[case((50, -51), (99, 1, 0))]
    fn test_apply_rotation_from_50_edge_cases(
        #[case] (starting_position, rotation): (isize, isize),
        #[case] (ending_position, times_crossed_zero, times_stopped_at_zero): (isize, isize, isize),
    ) {
        assert_eq!(
            apply_rotation(starting_position, rotation),
            (ending_position, times_crossed_zero, times_stopped_at_zero)
        );
    }

    #[rstest]
    #[case((0, 1), (1, 0, 0))]
    #[case((0, -1), (99, 0, 0))]
    #[case((0, 99), (99, 0, 0))]
    #[case((0, -99), (1, 0, 0))]
    #[case((0, 100), (0, 0, 1))]
    #[case((0, -100), (0, 0, 1))]
    #[case((0, -101), (99, 1, 0))]
    #[case((0, 101), (1, 1, 0))]
    fn test_apply_rotation_from_0_edge_cases(
        #[case] (starting_position, rotation): (isize, isize),
        #[case] (ending_position, times_crossed_zero, times_stopped_at_zero): (isize, isize, isize),
    ) {
        assert_eq!(
            apply_rotation(starting_position, rotation),
            (ending_position, times_crossed_zero, times_stopped_at_zero)
        );
    }

    #[rstest]
    #[case((99, 1), (0, 0, 1))]
    #[case((99, 100), (99, 1, 0))]
    #[case((99, -100), (99, 1, 0))]
    fn test_apply_rotation_from_99_edge_cases(
        #[case] (starting_position, rotation): (isize, isize),
        #[case] (ending_position, times_crossed_zero, times_stopped_at_zero): (isize, isize, isize),
    ) {
        assert_eq!(
            apply_rotation(starting_position, rotation),
            (ending_position, times_crossed_zero, times_stopped_at_zero)
        );
    }

    #[rstest]
    #[case((50, 50), (0, 0, 1))]
    #[case((50, -50), (0, 0, 1))]
    #[case((50, 150), (0, 1, 1))]
    #[case((50, -150), (0, 1, 1))]
    #[case((50, 250), (0, 2, 1))]
    #[case((50, -250), (0, 2, 1))]
    #[case((50, 249), (99, 2, 0))]
    #[case((50, -249), (1, 2, 0))]
    #[case((50, 251), (1, 3, 0))]
    #[case((50, -251), (99, 3, 0))]
    #[case((50, 10_000), (50, 100, 0))]
    #[case((50, -10_000), (50, 100, 0))]
    fn test_apply_rotation_from_50_large(
        #[case] (starting_position, rotation): (isize, isize),
        #[case] (ending_position, times_crossed_zero, times_stopped_at_zero): (isize, isize, isize),
    ) {
        assert_eq!(
            apply_rotation(starting_position, rotation),
            (ending_position, times_crossed_zero, times_stopped_at_zero)
        );
    }

    #[rstest]
    #[case((0, 200), (0, 1, 1))]
    #[case((0, -200), (0, 1, 1))]
    fn test_apply_rotation_from_0_large(
        #[case] (starting_position, rotation): (isize, isize),
        #[case] (ending_position, times_crossed_zero, times_stopped_at_zero): (isize, isize, isize),
    ) {
        assert_eq!(
            apply_rotation(starting_position, rotation),
            (ending_position, times_crossed_zero, times_stopped_at_zero)
        );
    }

    #[rstest]
    #[case((99, 200), (99, 2, 0))]
    #[case((99, -200), (99, 2, 0))]
    fn test_apply_rotation_from_99_large(
        #[case] (starting_position, rotation): (isize, isize),
        #[case] (ending_position, times_crossed_zero, times_stopped_at_zero): (isize, isize, isize),
    ) {
        assert_eq!(
            apply_rotation(starting_position, rotation),
            (ending_position, times_crossed_zero, times_stopped_at_zero)
        );
    }

    #[rstest]
    #[case((1, 100), (1, 1, 0))]
    #[case((1, 200), (1, 2, 0))]
    #[case((1, -100), (1, 1, 0))]
    #[case((1, -200), (1, 2, 0))]
    fn test_apply_rotation_from_1_large(
        #[case] (starting_position, rotation): (isize, isize),
        #[case] (ending_position, times_crossed_zero, times_stopped_at_zero): (isize, isize, isize),
    ) {
        assert_eq!(
            apply_rotation(starting_position, rotation),
            (ending_position, times_crossed_zero, times_stopped_at_zero)
        );
    }

    #[rstest]
    #[case((50, 10), (60, 0, 0))]
    #[case((50, -10), (40, 0, 0))]
    fn test_apply_rotation_not_crossing_zero(
        #[case] (starting_position, rotation): (isize, isize),
        #[case] (ending_position, times_crossed_zero, times_stopped_at_zero): (isize, isize, isize),
    ) {
        assert_eq!(
            apply_rotation(starting_position, rotation),
            (ending_position, times_crossed_zero, times_stopped_at_zero)
        );
    }
}
