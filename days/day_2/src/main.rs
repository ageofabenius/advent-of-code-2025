use const_utils::{load_file_and_split_text, number_parsing::parse_usize};

load_file_and_split_text!("../input.txt", ',');

const fn parse_range(s: &str) -> (usize, usize) {
    // Find the index of the '-' character
    let bytes = s.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'-' {
            break;
        }
        index += 1;
    }

    // Now, parse both numbers
    let left = parse_usize(&bytes, None, Some(index));
    let right = parse_usize(&bytes, Some(index + 1), None);

    (left, right)
}

/// Since we're in const, we can't build up a String or &str as we traverse
/// through the number's digits.  We can build up an array, but we need to
/// determine the capacity needed at runtime.  I'm going to make an assumption
/// that none of the numbers in the input will be bigger than u128::MAX =
/// 340282366920938463463374607431768211455, this is 39 digits long so we'll
/// allocate an array of capacity 39 and keep track of its length manually.
const DIGIT_ARRAY_CAPACITY: usize = 39;
const fn number_to_digit_array(number: usize) -> ([u8; DIGIT_ARRAY_CAPACITY], usize) {
    let mut digit_array: [u8; DIGIT_ARRAY_CAPACITY] = [0; DIGIT_ARRAY_CAPACITY];
    let mut length = 0;

    let mut truncated_number = number;

    // We now loop
    let mut i = 0;
    while i <= DIGIT_ARRAY_CAPACITY {
        // The least-significant digit is simply the rest of the number % 10
        let right_most_digit: u8 = (truncated_number % 10) as u8;

        // We add the right-most digit to the digit_array
        digit_array[DIGIT_ARRAY_CAPACITY - 1 - length] = right_most_digit;
        length += 1;

        // We then integer divide the rest of the number by 10 to shift right
        truncated_number = truncated_number / 10;

        // Finally, if there is nothing left we break
        if truncated_number == 0 {
            break;
        }

        i += 1;
    }

    return (digit_array, length);
}

const fn digit_array_repeats((digit_array, length): ([u8; DIGIT_ARRAY_CAPACITY], usize)) -> bool {
    // First, we check if the length is odd
    if length % 2 != 0 {
        return false;
    }

    let half_length = length / 2;
    let right_start_index = DIGIT_ARRAY_CAPACITY - 1;
    let left_start_index = DIGIT_ARRAY_CAPACITY - 1 - half_length;

    let mut i = 0;
    while i < half_length {
        let left_digit = digit_array[left_start_index - i];
        let right_digit = digit_array[right_start_index - i];
        if left_digit != right_digit {
            return false;
        }
        i += 1;
    }

    true
}

/// Iterates through the range, finding all numbers with repeating digits, and
/// returns their sum
const fn sum_repeating_digit_numbers_in_range(start: usize, end: usize) -> usize {
    let mut sum = 0;

    let mut i = start;
    while i <= end {
        let is_repeating = digit_array_repeats(number_to_digit_array(i));
        if is_repeating {
            sum += i;
        };

        i += 1;
    }

    sum
}

const fn sum_repeating_digit_numbers_in_ranges(s: [&str; NUM_LINES]) -> usize {
    let mut sum = 0;
    let mut i = 0;
    while i < s.len() {
        let line = s[i];
        let (range_start, range_end) = parse_range(line);
        let sum_of_repeating_numbers_in_range =
            sum_repeating_digit_numbers_in_range(range_start, range_end);
        sum += sum_of_repeating_numbers_in_range;
        i += 1;
    }
    sum
}

fn main() {
    const PART_1_ANSWER: usize = sum_repeating_digit_numbers_in_ranges(INPUT_LINES);
    println!("PART_1_ANSWER: {}", PART_1_ANSWER);
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("11-22", (11, 22))]
    #[case("95-115", (95,115))]
    #[case("998-1012", (998,1012))]
    #[case("1188511880-1188511890", (1188511880,1188511890))]
    #[case("222220-222224", (222220,222224))]
    #[case("1698522-1698528", (1698522,1698528))]
    #[case("446443-446449", (446443,446449))]
    #[case("38593856-38593862", (38593856,38593862))]
    fn test_parse_range(#[case] input: &str, #[case] expected: (usize, usize)) {
        assert_eq!(parse_range(input), expected);
    }

    #[rstest]
    #[case(11, ([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1], 2))]
    #[case(121, ([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 1], 3))]
    #[case(1234, ([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4], 4))]
    fn test_number_to_digit_array(
        #[case] number: usize,
        #[case] digits: ([u8; DIGIT_ARRAY_CAPACITY], usize),
    ) {
        assert_eq!(number_to_digit_array(number), digits);
    }

    #[rstest]
    #[case(11, true)]
    #[case(121, false)]
    #[case(1212, true)]
    #[case(1234, false)]
    #[case(1188511885, true)]
    fn test_digit_array_repeats(#[case] number: usize, #[case] repeats: bool) {
        assert_eq!(digit_array_repeats(number_to_digit_array(number)), repeats);
    }

    #[rstest]
    #[case("11-22", &[11, 22])]
    #[case("95-115", &[99])]
    #[case("998-1012", &[1010])]
    #[case("1188511880-1188511890", &[1188511885])]
    #[case("222220-222224", &[222222])]
    #[case("1698522-1698528", &[])]
    fn test_sum_repeating_digit_numbers_in_range(#[case] s: &str, #[case] numbers: &[usize]) {
        let (start, end) = parse_range(s);
        assert_eq!(
            sum_repeating_digit_numbers_in_range(start, end),
            numbers.iter().sum()
        );
    }
}
