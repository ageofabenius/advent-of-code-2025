use const_utils::{load_file_and_split_text, number_parsing::parse_usize};

load_file_and_split_text!("../input.txt", '\n');

const fn parse_rotation(s: &str) -> isize {
    let bytes = s.as_bytes();
    let sign = match bytes[0] {
        b'L' => -1,
        b'R' => 1,
        _ => unreachable!(),
    };
    let number: isize = parse_usize(bytes, Some(1), None) as isize;

    sign * number
}

const fn part_1(lines: &[&str]) -> isize {
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
const PART_1_ANSWER: isize = part_1(&INPUT_LINES);

const fn part_2(lines: &[&str]) -> isize {
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
const fn apply_rotation(starting_position: isize, rotation: isize) -> (isize, isize, isize) {
    // Determine the number of full rotations
    let full_rotations = (rotation / 100).abs();

    // We will apply the remaining rotation
    let remaining_rotation = rotation % 100;

    if remaining_rotation == 0 {
        // We don't need to apply rotations, but we do need to account for
        // ending on 0 as we return that separately
        if starting_position == 0 {
            return (starting_position, full_rotations - 1, 1);
        } else {
            return (starting_position, full_rotations, 0);
        }
    }

    let position_with_wrapping = starting_position + remaining_rotation;

    let times_crossed_zero = if position_with_wrapping > 100 {
        1
    } else if (position_with_wrapping < 0) && (starting_position != 0) {
        1
    } else {
        0
    };

    // Adjust position to be from -99 to 99
    let mut adjusted_position = position_with_wrapping % 100;
    // Adjust position to be from 0 to 99
    if adjusted_position < 0 {
        adjusted_position += 100;
    }

    let times_stopped_at_zero = if adjusted_position == 0 { 1 } else { 0 };

    (
        adjusted_position,
        full_rotations + times_crossed_zero,
        times_stopped_at_zero,
    )
}

const PART_2_ANSWER: isize = part_2(&INPUT_LINES);

fn main() {
    println!("PART_1_ANSWER: {PART_1_ANSWER}");
    println!("PART_2_ANSWER: {PART_2_ANSWER}");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_parse_rotation() {
        assert_eq!(parse_rotation("L31"), -31);
        assert_eq!(parse_rotation("R43"), 43);
    }

    #[test]
    fn test_do_the_thing() {
        assert_eq!(part_1(&["L50"]), 1);
        assert_eq!(part_1(&["R50"]), 1);

        assert_eq!(part_1(&["R50", "R100"]), 2);
    }

    #[test]
    fn test_do_the_second_thing() {
        assert_eq!(part_2(&["R50"]), 1);
        assert_eq!(part_2(&["R50", "R50"]), 1);
        assert_eq!(part_2(&["R50", "R50", "R50"]), 2);

        assert_eq!(part_2(&["L50"]), 1);
        assert_eq!(part_2(&["L50", "L50"]), 1);
        assert_eq!(part_2(&["L50", "L50", "L50"]), 2);
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
