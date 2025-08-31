use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<usize> {
    use aoc_parse::{parser, prelude::*};
    let parser = parser!(repeat_sep(usize, ' '));
    parser.parse(input).unwrap()
}

fn sum_metadata_entries(node: &[usize]) -> (usize, &[usize]) {
    let child_nodes_qty = node[0];
    let metadata_entries_qty = node[1];

    let mut metadata_sum = 0;
    let mut tail = &node[2..];

    for _ in 0..child_nodes_qty {
        let (sub_sum, remaining_numbers) = sum_metadata_entries(tail);
        metadata_sum += sub_sum;
        tail = remaining_numbers;
    }

    metadata_sum += tail[0..metadata_entries_qty].iter().sum::<usize>();

    (metadata_sum, &tail[metadata_entries_qty..])
}

#[aoc(day8, part1)]
fn part1(numbers: &[usize]) -> usize {
    sum_metadata_entries(numbers).0
}

fn node_value(node: &[usize]) -> (usize, &[usize]) {
    let child_nodes_qty = node[0];
    let metadata_entries_qty = node[1];

    if child_nodes_qty == 0 {
        (
            node[2..2 + metadata_entries_qty].iter().sum::<usize>(),
            &node[2 + metadata_entries_qty..],
        )
    } else {
        let mut child_nodes_values = Vec::new();
        let mut tail = &node[2..];
        let mut value = 0;

        for _ in 0..child_nodes_qty {
            let (sub_value, remaining_numbers) = node_value(tail);

            child_nodes_values.push(sub_value);
            tail = remaining_numbers;
        }

        for metadata_entry in tail[0..metadata_entries_qty].iter() {
            if *metadata_entry > 0 && *metadata_entry <= child_nodes_qty {
                value += child_nodes_values[*metadata_entry - 1];
            }
        }

        (value, &tail[metadata_entries_qty..])
    }
}

#[aoc(day8, part2)]
fn part2(numbers: &[usize]) -> usize {
    node_value(numbers).0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 138);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 66);
    }
}
