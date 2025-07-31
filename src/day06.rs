use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<(i32, i32)> {
    use aoc_parse::{parser, prelude::*};
    let parser = parser!(lines(i32 ", " i32));
    parser.parse(input).unwrap()
}

#[aoc(day6, part1)]
fn part1(coordinates: &[(i32, i32)]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 17);
    }
}
