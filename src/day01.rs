use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> i32 {
    use aoc_parse::{parser, prelude::*};

    unimplemented!()
}

#[aoc(day1, part1)]
fn part1(input: &i32) -> i32 {
    unimplemented!()
}

// #[aoc(day9, part2)]
// fn part2(input: &i32) -> i32 {
//     unimplemented!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 11);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse_input(TEST_INPUT)), 31);
    // }
}
