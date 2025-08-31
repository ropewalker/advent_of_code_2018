use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{BTreeMap, BTreeSet, HashSet};

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<(char, char)> {
    use aoc_parse::{parser, prelude::*};
    let parser = parser!(lines("Step " alpha " must be finished before step " alpha " can begin."));
    parser.parse(input).unwrap()
}

fn next_and_prev_steps(
    dependencies: &[(char, char)],
) -> (
    BTreeMap<char, BTreeSet<char>>,
    BTreeMap<char, HashSet<char>>,
) {
    dependencies.iter().fold(
        (BTreeMap::new(), BTreeMap::new()),
        |(mut next_steps, mut prev_steps), (prev_step, next_step)| {
            next_steps.entry(*prev_step).or_default().insert(*next_step);
            prev_steps.entry(*next_step).or_default().insert(*prev_step);
            (next_steps, prev_steps)
        },
    )
}

#[aoc(day7, part1)]
fn part1(dependencies: &[(char, char)]) -> String {
    let (mut next_steps, mut prev_steps): (
        BTreeMap<char, BTreeSet<char>>,
        BTreeMap<char, HashSet<char>>,
    ) = next_and_prev_steps(dependencies);

    let mut result = String::new();

    while !next_steps.is_empty() {
        let step = *next_steps
            .keys()
            .find(|step| !prev_steps.contains_key(step) || prev_steps.get(step).unwrap().is_empty())
            .unwrap();
        let following_steps = next_steps.remove(&step).unwrap();

        for following_step in following_steps.iter() {
            prev_steps.entry(*following_step).or_default().remove(&step);
        }

        prev_steps.remove(&step);
        result.push(step);
    }

    for step in prev_steps.keys() {
        result.push(*step);
    }

    result
}

fn step_duration(step: &char, base_step_duration: u32) -> u32 {
    *step as u32 - 'A' as u32 + 1 + base_step_duration
}

#[derive(Clone)]
struct Worker {
    current_step: Option<char>,
    remaining_duration: u32,
}

fn total_duration(
    dependencies: &[(char, char)],
    num_workers: usize,
    base_step_duration: u32,
) -> u32 {
    let (next_steps, prev_steps): (
        BTreeMap<char, BTreeSet<char>>,
        BTreeMap<char, HashSet<char>>,
    ) = next_and_prev_steps(dependencies);

    let mut in_progress: HashSet<char> = HashSet::new();
    let mut finished: HashSet<char> = HashSet::new();

    let mut workers = vec![
        Worker {
            current_step: None,
            remaining_duration: 0
        };
        num_workers
    ];

    let mut total_duration = 0;

    loop {
        for worker in workers.iter_mut() {
            if worker.current_step.is_none() {
                if let Some(step) = next_steps
                    .keys()
                    .find(|step| {
                        !in_progress.contains(step)
                            && !finished.contains(step)
                            && (!prev_steps.contains_key(step)
                                || prev_steps
                                    .get(step)
                                    .unwrap()
                                    .iter()
                                    .all(|prerequisite| finished.contains(prerequisite)))
                    })
                    .cloned()
                {
                    in_progress.insert(step);
                    worker.current_step = Some(step);
                    worker.remaining_duration = step_duration(&step, base_step_duration);
                } else if let Some(step) = prev_steps
                    .keys()
                    .find(|step| {
                        !in_progress.contains(step)
                            && !finished.contains(step)
                            && prev_steps
                                .get(step)
                                .unwrap()
                                .iter()
                                .all(|prerequisite| finished.contains(prerequisite))
                    })
                    .cloned()
                {
                    in_progress.insert(step);
                    worker.current_step = Some(step);
                    worker.remaining_duration = step_duration(&step, base_step_duration);
                }
            }
        }

        if let Some(min_duration) = workers
            .iter()
            .filter_map(|worker| {
                if worker.current_step.is_some() {
                    Some(worker.remaining_duration)
                } else {
                    None
                }
            })
            .min()
        {
            total_duration += min_duration;

            for worker in workers.iter_mut() {
                if worker.current_step.is_some() {
                    worker.remaining_duration -= min_duration;

                    if worker.remaining_duration == 0 {
                        in_progress.remove(&worker.current_step.unwrap());
                        finished.insert(worker.current_step.unwrap());
                        worker.current_step = None;
                    }
                }
            }
        } else {
            break;
        }
    }

    total_duration
}

#[aoc(day7, part2)]
fn part2(dependencies: &[(char, char)]) -> u32 {
    total_duration(dependencies, 5, 60)
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

    #[test]
    fn part2_example() {
        assert_eq!(total_duration(&parse_input(TEST_INPUT), 2, 0), 15);
    }
}
