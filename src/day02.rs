use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

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

#[aoc(day2, part2)]
fn part2(ids: &[String]) -> String {
    let mut prefixes_and_suffixes: HashSet<(&str, &str)> = HashSet::new();

    for id in ids.iter() {
        for i in 0..id.len() {
            if let Some((prefix, suffix)) = prefixes_and_suffixes.get(&(&id[..i], &id[i + 1..])) {
                let mut output = prefix.to_string();
                output.push_str(suffix);

                return output;
            } else {
                prefixes_and_suffixes.insert((&id[..i], &id[i + 1..]));
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";

    static TEST_INPUT_2: &str = r"abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 12);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), "fgij".to_string());
    }
}
