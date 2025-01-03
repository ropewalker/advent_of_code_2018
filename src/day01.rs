use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .flat_map(|line| line.split(", ").map(|number| number.parse().unwrap()))
        .collect()
}

#[aoc(day1, part1)]
fn part1(frequency_changes: &[i32]) -> i32 {
    frequency_changes.iter().sum()
}

#[aoc(day1, part2)]
fn part2(frequency_changes: &[i32]) -> i32 {
    let mut frequency = 0;
    let mut frequencies = HashSet::from([frequency]);

    for frequency_change in frequency_changes.iter().cycle() {
        frequency += *frequency_change;

        if frequencies.contains(&frequency) {
            return frequency;
        } else {
            frequencies.insert(frequency);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = "+1, -2, +3, +1";
    static TEST_INPUT_2: &str = "+1, +1, +1";
    static TEST_INPUT_3: &str = "+1, +1, -2";
    static TEST_INPUT_4: &str = "-1, -2, -3";
    static TEST_INPUT_5: &str = "+1, -1";
    static TEST_INPUT_6: &str = "+3, +3, +4, -2, -4";
    static TEST_INPUT_7: &str = "-6, +3, +8, +5, -6";
    static TEST_INPUT_8: &str = "+7, +7, -2, -7, -4";

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 3);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 3);
    }

    #[test]
    fn part1_example_3() {
        assert_eq!(part1(&parse_input(TEST_INPUT_3)), 0);
    }

    #[test]
    fn part1_example_4() {
        assert_eq!(part1(&parse_input(TEST_INPUT_4)), -6);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&parse_input(TEST_INPUT_1)), 2);
    }

    #[test]
    fn part2_example_5() {
        assert_eq!(part2(&parse_input(TEST_INPUT_5)), 0);
    }

    #[test]
    fn part2_example_6() {
        assert_eq!(part2(&parse_input(TEST_INPUT_6)), 10);
    }

    #[test]
    fn part2_example_7() {
        assert_eq!(part2(&parse_input(TEST_INPUT_7)), 5);
    }

    #[test]
    fn part2_example_8() {
        assert_eq!(part2(&parse_input(TEST_INPUT_8)), 14);
    }
}
