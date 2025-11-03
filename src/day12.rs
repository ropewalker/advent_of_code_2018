use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Pot {
    Plant,
    Empty,
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> (Vec<Pot>, HashMap<[Pot; 5], Pot>) {
    use Pot::*;
    use aoc_parse::{parser, prelude::*};
    let parser = parser!(
        rule pot: Pot = {'#' => Plant, '.' => Empty};

        section(line("initial state: " pot+))
        section(hash_map(lines(ll:pot l:pot c:pot r:pot rr:pot " => " n:pot
            => ([ll, l, c, r, rr], n))))
    );
    parser.parse(input).unwrap()
}

fn sum_of_pot_numbers_after(
    initial_state: &[Pot],
    notes: &HashMap<[Pot; 5], Pot>,
    generations_num: usize,
) -> i32 {
    use Pot::*;

    let mut state = VecDeque::from(initial_state.to_vec());

    let mut shift = 0;
    let mut generation = 0;

    while generation < generations_num {
        for _ in 0..4 {
            state.push_front(Empty);
            state.push_back(Empty);
        }

        shift += 4;

        let mut new_state = state.clone();

        for index in 2..new_state.len() - 2 {
            let key: [Pot; 5] = [
                state[index - 2],
                state[index - 1],
                state[index],
                state[index + 1],
                state[index + 2],
            ];

            if notes.contains_key(&key) {
                new_state[index] = *notes.get(&key).unwrap();
            } else {
                new_state[index] = Empty;
            }
        }

        state = new_state;

        generation += 1;

        while state.front() == Some(&Empty) {
            state.pop_front();
            shift -= 1;
        }

        while state.back() == Some(&Empty) {
            state.pop_back();
        }
    }

    state
        .iter()
        .enumerate()
        .filter_map(|(index, pot)| {
            if *pot == Plant {
                Some(index as i32 - shift)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day12, part1)]
fn part1((initial_state, notes): &(Vec<Pot>, HashMap<[Pot; 5], Pot>)) -> i32 {
    sum_of_pot_numbers_after(initial_state, notes, 20)
}

#[aoc(day12, part2)]
fn part2((initial_state, notes): &(Vec<Pot>, HashMap<[Pot; 5], Pot>)) -> i32 {
    sum_of_pot_numbers_after(initial_state, notes, 50_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 325);
    }
}
