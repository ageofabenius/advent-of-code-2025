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

fn main() {
    println!("Hello, world!");
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
}
