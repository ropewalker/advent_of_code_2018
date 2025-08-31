use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_input(input: &str) -> (usize, usize) {
    use aoc_parse::{parser, prelude::*};
    let parser = parser!(usize " players; last marble is worth " usize " points");
    parser.parse(input).unwrap()
}

#[aoc(day9, part1)]
fn part1((num_players, last_marble_score): &(usize, usize)) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_0: &str = "9 players; last marble is worth 25 points";
    static TEST_INPUT_1: &str = "10 players; last marble is worth 1618 points";
    static TEST_INPUT_2: &str = "13 players; last marble is worth 7999 points";
    static TEST_INPUT_3: &str = "17 players; last marble is worth 1104 points";
    static TEST_INPUT_4: &str = "21 players; last marble is worth 6111 points";
    static TEST_INPUT_5: &str = "30 players; last marble is worth 5807 points";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT_0)), 32);
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 8_317);
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 146_373);
        assert_eq!(part1(&parse_input(TEST_INPUT_3)), 2_764);
        assert_eq!(part1(&parse_input(TEST_INPUT_4)), 54_718);
        assert_eq!(part1(&parse_input(TEST_INPUT_5)), 37_305);
    }
}
