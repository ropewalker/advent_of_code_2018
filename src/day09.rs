use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_input(input: &str) -> (usize, usize) {
    use aoc_parse::{parser, prelude::*};
    let parser = parser!(usize " players; last marble is worth " usize " points");
    parser.parse(input).unwrap()
}

fn winning_score(num_players: usize, last_marble_score: usize) -> usize {
    let mut scores = vec![0usize; num_players];

    let mut positions = vec![Some(0)];
    let mut marbles_in_circle = 1usize;
    let mut current_position = 0usize;

    for turn in 1..=last_marble_score {
        if turn % 23 != 0 {
            let new_position = (current_position + 1) % marbles_in_circle + 1;

            positions.iter_mut().for_each(|position| {
                if let Some(position) = position {
                    if *position >= new_position {
                        *position += 1;
                    }
                }
            });

            positions.push(Some(new_position));
            marbles_in_circle += 1;

            current_position = new_position;
        } else {
            scores[turn % num_players] += turn;
            positions.push(None);

            let (marble_score, removed_position) = positions
                .iter()
                .enumerate()
                .find(|(_, position)| {
                    position.is_some()
                        && position.unwrap()
                            == (current_position + marbles_in_circle - 7) % marbles_in_circle
                })
                .unwrap();

            scores[turn % num_players] += marble_score;

            let removed_position = removed_position.to_owned().unwrap();

            positions[marble_score] = None;
            marbles_in_circle -= 1;

            positions.iter_mut().for_each(|position| {
                if let Some(position) = position {
                    if *position > removed_position {
                        *position -= 1;
                    }
                }
            });

            current_position = removed_position;
        }
    }

    *scores.iter().max().unwrap()
}

#[aoc(day9, part1)]
fn part1((num_players, last_marble_score): &(usize, usize)) -> usize {
    winning_score(*num_players, *last_marble_score)
}

#[aoc(day9, part2)]
fn part2((num_players, last_marble_score): &(usize, usize)) -> usize {
    winning_score(*num_players, *last_marble_score * 100)
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

    fn part1_example_0() {
        assert_eq!(part1(&parse_input(TEST_INPUT_0)), 32);
    }

    fn part1_example_1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 8_317);
    }

    fn part1_example_2() {
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 146_373);
    }

    fn part1_example_3() {
        assert_eq!(part1(&parse_input(TEST_INPUT_3)), 2_764);
    }

    fn part1_example_4() {
        assert_eq!(part1(&parse_input(TEST_INPUT_4)), 54_718);
    }

    fn part1_example_5() {
        assert_eq!(part1(&parse_input(TEST_INPUT_5)), 37_305);
    }
}
