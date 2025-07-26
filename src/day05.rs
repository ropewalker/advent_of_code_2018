use aoc_runner_derive::aoc;
use std::collections::HashMap;

fn add_unit(polymer: &mut Vec<char>, unit: char) {
    if polymer.is_empty() {
        polymer.push(unit);
    } else {
        let last = polymer.last().unwrap();

        if last.eq_ignore_ascii_case(&unit)
            && (last.is_lowercase() && unit.is_uppercase()
                || last.is_uppercase() && unit.is_lowercase())
        {
            polymer.pop();
        } else {
            polymer.push(unit);
        }
    }
}

#[aoc(day5, part1)]
fn part1(polymer: &str) -> usize {
    let mut stack = Vec::new();

    for unit in polymer.chars() {
        add_unit(&mut stack, unit);
    }

    stack.len()
}

#[aoc(day5, part2)]
fn part2(polymer: &str) -> usize {
    let mut stacks = HashMap::new();

    for unit in 'a'..='z' {
        stacks.insert(unit, Vec::new());
    }

    for unit in polymer.chars() {
        for (index, stack) in stacks.iter_mut() {
            if unit.eq_ignore_ascii_case(index) {
                continue;
            }

            add_unit(stack, unit);
        }
    }

    stacks
        .values()
        .min_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST_INPUT), 10);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}
