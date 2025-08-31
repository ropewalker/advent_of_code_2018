use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<(char, char)> {
    use aoc_parse::{parser, prelude::*};
    let parser = parser!(lines("Step " alpha " must be finished before step " alpha " can begin."));
    parser.parse(input).unwrap()
}

#[aoc(day7, part1)]
fn part1(dependencies: &[(char, char)]) -> String {
    let (mut next_steps, mut prev_steps): (
        BTreeMap<char, BTreeSet<char>>,
        BTreeMap<char, HashSet<char>>,
    ) = dependencies.iter().fold(
        (BTreeMap::new(), BTreeMap::new()),
        |(mut next_steps, mut prev_steps), (prev_step, next_step)| {
            next_steps.entry(*prev_step).or_default().insert(*next_step);
            prev_steps.entry(*next_step).or_default().insert(*prev_step);
            (next_steps, prev_steps)
        },
    );

    let mut result = String::new();

    while !next_steps.is_empty() {
        let prev_step = *next_steps
            .keys()
            .find(|prev_step| {
                !prev_steps.contains_key(prev_step)
                    || prev_steps.get(&prev_step).unwrap().is_empty()
            })
            .unwrap();
        let (prev_step, following_steps) = next_steps.remove_entry(&prev_step).unwrap();

        for following_step in following_steps.iter() {
            prev_steps
                .entry(*following_step)
                .or_default()
                .remove(&prev_step);
        }

        prev_steps.remove(&prev_step);

        result.push(prev_step);
    }

    for step in prev_steps.keys() {
        result.push(*step);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), "CABDFE".to_string());
    }
}
