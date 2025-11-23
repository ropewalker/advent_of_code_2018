use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day14)]
fn parse_input(input: &str) -> usize {
    input.parse().unwrap()
}

#[aoc(day14, part1)]
fn part1(num_recipes: &usize) -> String {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = "9";
    static TEST_INPUT_2: &str = "5";
    static TEST_INPUT_3: &str = "18";
    static TEST_INPUT_4: &str = "2018";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), "5158916779");
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), "0124515891");
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1(&parse_input(TEST_INPUT_3)), "9251071085");
    }

    #[test]
    fn part1_example4() {
        assert_eq!(part1(&parse_input(TEST_INPUT_4)), "5941429882");
    }
}
