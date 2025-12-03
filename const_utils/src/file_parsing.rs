/// At compile time, loads a given file and splits it at the give delimiter.
///
/// The resulting fixed-size array of &str is named `INPUT_LINES`.
///
/// Example usage:
/// ```ignore
/// load_file_and_split_text!("path/to/file.txt", '\n');
/// ```
#[macro_export]
macro_rules! load_file_and_split_text {
    ($file_path:expr, $delimiter:expr) => {
        const INPUT: &str = include_str!($file_path);

        /// Count the lines, this is needed for the const definition of `INPUT_LINES` as
        /// a fixed-size array
        const NUM_LINES: usize = count_lines(INPUT);

        const fn count_lines(s: &str) -> usize {
            let bytes = s.as_bytes();
            let mut count = 0;
            let mut i = 0;
            let d = $delimiter as u8;
            while i < bytes.len() {
                if bytes[i] == d {
                    count += 1;
                }
                i += 1;
            }

            count += 1; // Add one to account for the last line, we assume there is never a trailing delimiter

            count
        }

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
                        let line_slice: &str = core::str::from_utf8_unchecked(
                            core::slice::from_raw_parts(ptr_line_start, line_length),
                        );

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

                // Handle the case where the string does not end with a
                // delimiter, we still need to capture the last line
                if i == s.len() && bytes[i - 1] != d {
                    unsafe {
                        let ptr_line_start: *const u8 = s.as_ptr().add(line_start);
                        let line_length = i - line_start;
                        let line_slice: &str = core::str::from_utf8_unchecked(
                            core::slice::from_raw_parts(ptr_line_start, line_length),
                        );

                        lines[line_num] = line_slice;
                    }
                }
            }

            lines
        }
        /// Define `INPUT_LINES` as a fixed-size array, and split the input into it
        const INPUT_LINES: [&str; NUM_LINES] = split_lines(INPUT, $delimiter);
    };
}

#[cfg(test)]
mod test_day_1 {
    load_file_and_split_text!("../../days/day_1/input.txt", '\n');

    #[test]
    fn test_foo() {
        // println!("{:?}", INPUT_LINES);
        assert_eq!(INPUT_LINES.len(), 4545);
        assert_eq!(
            INPUT_LINES[0..10],
            [
                "R27", "R13", "L8", "R30", "R22", "L9", "L32", "R22", "R20", "R16",
            ]
        );

        assert_eq!(INPUT_LINES[4540..], ["R19", "R3", "R6", "L15", "L3",]);
    }
}

#[cfg(test)]
mod test_day_2 {
    load_file_and_split_text!("../../days/day_2/input.txt", ',');

    #[test]
    fn test_foo() {
        // println!("{:?}", INPUT_LINES);
        assert_eq!(INPUT_LINES.len(), 32);
        assert_eq!(
            INPUT_LINES[0..3],
            [
                "269194394-269335492",
                "62371645-62509655",
                "958929250-958994165"
            ]
        );

        assert_eq!(INPUT_LINES[30..], ["941928989-941964298", "3416-9716"]);
    }
}
