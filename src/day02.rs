use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[aoc(day2, part1)]
fn part1(ids: &[String]) -> usize {
    let (contains_exactly_two_of_any_letter_count, contains_exactly_three_of_any_letter_count) =
        ids.iter().fold(
            (0, 0),
            |(
                mut contains_exactly_two_of_any_letter_count,
                mut contains_exactly_three_of_any_letter_count,
            ),
             id| {
                let mut letter_counts: HashMap<char, usize> = HashMap::new();

                for letter in id.chars() {
                    *letter_counts.entry(letter).or_default() += 1;
                }

                if letter_counts.values().any(|count| *count == 2) {
                    contains_exactly_two_of_any_letter_count += 1;
                }

                if letter_counts.values().any(|count| *count == 3) {
                    contains_exactly_three_of_any_letter_count += 1;
                }

                (
                    contains_exactly_two_of_any_letter_count,
                    contains_exactly_three_of_any_letter_count,
                )
            },
        );

    contains_exactly_two_of_any_letter_count * contains_exactly_three_of_any_letter_count
}

// #[aoc(day2, part2)]
// fn part2(input: &i32) -> i32 {
//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 12);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse_input(TEST_INPUT)), 31);
    // }
}
