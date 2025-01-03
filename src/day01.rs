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
    let frequency_change_per_cycle: i32 = frequency_changes.iter().sum();

    if frequency_change_per_cycle == 0 {
        let mut frequency = 0;
        let mut frequencies = HashSet::from([frequency]);

        for frequency_change in frequency_changes.iter() {
            frequency += frequency_change;

            if frequencies.contains(&frequency) {
                return frequency;
            } else {
                frequencies.insert(frequency);
            }
        }
    }

    let frequencies = frequency_changes
        .iter()
        .take(frequency_changes.len() - 1)
        .fold(vec![0], |mut frequencies, frequency_change| {
            frequencies.push(frequencies.last().unwrap() + *frequency_change);
            frequencies
        });

    let mut frequencies_by_modulo: Vec<Vec<(i32, usize)>> = frequencies.iter().enumerate().fold(
        vec![vec![]; frequency_change_per_cycle.unsigned_abs() as usize],
        |mut frequencies_by_modulo, (index, frequency)| {
            frequencies_by_modulo[frequency.rem_euclid(frequency_change_per_cycle) as usize]
                .push((*frequency, index));
            frequencies_by_modulo
        },
    );

    let (starting_frequency_index, number_of_cycles) = frequencies_by_modulo
        .iter_mut()
        .flat_map(|frequencies| {
            frequencies.sort_unstable_by(|(frequency_1, _), (frequency_2, _)| {
                if frequency_change_per_cycle > 0 {
                    frequency_1.cmp(frequency_2)
                } else {
                    frequency_2.cmp(frequency_1)
                }
            });

            frequencies.windows(2).map(|window| {
                (
                    window[0].1,
                    (window[1].0 - window[0].0) / frequency_change_per_cycle,
                )
            })
        })
        .min_by(
            |(index_1, number_of_cycles_1), (index_2, number_of_cycles_2)| {
                number_of_cycles_1
                    .cmp(number_of_cycles_2)
                    .then(index_1.cmp(index_2))
            },
        )
        .unwrap();

    frequencies[starting_frequency_index] + number_of_cycles * frequency_change_per_cycle
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
