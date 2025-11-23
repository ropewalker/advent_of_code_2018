use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Copy, Clone)]
enum Tile {
    Wall,
    OpenCavern,
}

const ATTACK_POWER: usize = 3;
const STARTING_HIT_POINTS: usize = 200;

#[derive(Copy, Clone)]
enum Race {
    Elf,
    Goblin,
}

#[derive(Copy, Clone)]
struct Unit {
    race: Race,
    attack_power: usize,
    hit_points: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl PartialOrd for Coordinates {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coordinates {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl From<(usize, usize)> for Coordinates {
    fn from(value: (usize, usize)) -> Self {
        Coordinates {
            x: value.0,
            y: value.1,
        }
    }
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> (Vec<Vec<Tile>>, BTreeMap<Coordinates, Unit>) {
    use Race::*;
    use Tile::*;

    let mut map = Vec::with_capacity(input.lines().count());
    let mut unit_starting_positions = BTreeMap::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::with_capacity(line.len());

        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => row.push(Wall),
                '.' => row.push(OpenCavern),
                'E' => {
                    row.push(OpenCavern);
                    unit_starting_positions.insert(
                        (x, y).into(),
                        Unit {
                            race: Elf,
                            attack_power: ATTACK_POWER,
                            hit_points: STARTING_HIT_POINTS,
                        },
                    );
                }
                'G' => {
                    row.push(OpenCavern);
                    unit_starting_positions.insert(
                        (x, y).into(),
                        Unit {
                            race: Goblin,
                            attack_power: ATTACK_POWER,
                            hit_points: STARTING_HIT_POINTS,
                        },
                    );
                }
                _ => unreachable!(),
            }
        }

        map.push(row);
    }

    (map, unit_starting_positions)
}

#[aoc(day15, part1)]
fn part1((map, unit_starting_positions): &(Vec<Vec<Tile>>, BTreeMap<Coordinates, Unit>)) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    static TEST_INPUT_2: &str = r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

    static TEST_INPUT_3: &str = r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

    static TEST_INPUT_4: &str = r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

    static TEST_INPUT_5: &str = r"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

    static TEST_INPUT_6: &str = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 27_730);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 36_334);
    }

    #[test]
    fn part1_example_3() {
        assert_eq!(part1(&parse_input(TEST_INPUT_3)), 39_514);
    }

    #[test]
    fn part1_example_4() {
        assert_eq!(part1(&parse_input(TEST_INPUT_4)), 27_755);
    }

    #[test]
    fn part1_example_5() {
        assert_eq!(part1(&parse_input(TEST_INPUT_5)), 28_944);
    }

    #[test]
    fn part1_example_6() {
        assert_eq!(part1(&parse_input(TEST_INPUT_6)), 18_740);
    }
}
