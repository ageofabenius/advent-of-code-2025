const INPUT: &str = include_str!("../input.txt");

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

const INPUT_LINES: [&str; NUM_LINES] = split_lines(INPUT, '\n');

const fn split_lines(s: &str, d: char) -> [&str; NUM_LINES] {
    let d = d as u8;
    let bytes = s.as_bytes();
    let mut lines: [&str; NUM_LINES] = [""; NUM_LINES];

    let mut line_num = 0;
    let mut i = 0;
    let mut line_start = 0;
    while i < bytes.len() {
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

            line_num += 1;
            line_start = i + 1;
        }

        i += 1;
    }

    lines
}

fn main() {
    // dbg!(INPUT);
    // dbg!(NUM_LINES);
    // dbg!(&INPUT_LINES[..10]);
}
