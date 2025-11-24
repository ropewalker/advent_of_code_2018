use crate::day15::Race::{Elf, Goblin};
use crate::day15::Tile::OpenCavern;
use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    OpenCavern,
}

const ATTACK_POWER: usize = 3;
const STARTING_HIT_POINTS: usize = 200;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Race {
    Elf,
    Goblin,
}

#[derive(Copy, Clone, Debug)]
struct Unit {
    race: Race,
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
fn parse_input(input: &str) -> (Vec<Vec<Tile>>, Vec<(Coordinates, Race)>) {
    use Race::*;
    use Tile::*;

    let mut map = Vec::with_capacity(input.lines().count());
    let mut unit_starting_positions = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::with_capacity(line.len());

        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => row.push(Wall),
                '.' => row.push(OpenCavern),
                'E' => {
                    row.push(OpenCavern);
                    unit_starting_positions.push(((x, y).into(), Elf));
                }
                'G' => {
                    row.push(OpenCavern);
                    unit_starting_positions.push(((x, y).into(), Goblin));
                }
                _ => unreachable!(),
            }
        }

        map.push(row);
    }

    (map, unit_starting_positions)
}

struct PathNode {
    starting_coordinates: Coordinates,
    current_coordinates: Coordinates,
    path_length: usize,
}

#[aoc(day15, part1)]
fn part1((map, unit_starting_positions): &(Vec<Vec<Tile>>, Vec<(Coordinates, Race)>)) -> usize {
    let mut unit_coordinates: HashMap<usize, Coordinates> = unit_starting_positions
        .iter()
        .enumerate()
        .map(|(id, (coordinates, _race))| (id, *coordinates))
        .collect();

    let mut units: Vec<Unit> = unit_starting_positions
        .iter()
        .map(|(_coordinates, race)| Unit {
            race: *race,
            hit_points: STARTING_HIT_POINTS,
        })
        .collect();

    let mut occupied_caverns: HashMap<Coordinates, usize> = unit_starting_positions
        .iter()
        .enumerate()
        .map(|(id, (coordinates, _race))| (*coordinates, id))
        .collect();

    let mut elves_count: usize = units.iter().filter(|unit| unit.race == Elf).count();
    let mut goblins_count: usize = units.iter().filter(|unit| unit.race == Goblin).count();

    let mut turn_order: Vec<usize> = (0..units.len()).collect();

    let mut full_rounds_count = 0;

    'main: while elves_count > 0 && goblins_count > 0 {
        for id in turn_order.iter() {
            if !unit_coordinates.contains_key(id) {
                continue;
            }

            let current_unit = units[*id];
            let current_unit_coordinates = unit_coordinates.get_mut(id).unwrap();

            let adjacent_tiles: [Coordinates; 4] = [
                (current_unit_coordinates.x, current_unit_coordinates.y - 1).into(),
                (current_unit_coordinates.x - 1, current_unit_coordinates.y).into(),
                (current_unit_coordinates.x + 1, current_unit_coordinates.y).into(),
                (current_unit_coordinates.x, current_unit_coordinates.y + 1).into(),
            ];

            let mut units_to_attack: Vec<usize> = Vec::with_capacity(4);

            for adjacent_tile in adjacent_tiles {
                if let Some(id) = occupied_caverns.get(&adjacent_tile)
                    && units[*id].race != current_unit.race
                {
                    units_to_attack.push(*id);
                }
            }

            if !units_to_attack.is_empty() {
                let min_health = units_to_attack
                    .iter()
                    .map(|id| units[*id].hit_points)
                    .min()
                    .unwrap();

                let unit_to_attack = units_to_attack
                    .iter()
                    .find(|id| units[**id].hit_points == min_health)
                    .unwrap();

                if units[*unit_to_attack].hit_points <= ATTACK_POWER {
                    units[*unit_to_attack].hit_points = 0;

                    let vacant_coordinates = unit_coordinates.get(unit_to_attack).unwrap();
                    occupied_caverns.remove(vacant_coordinates);
                    unit_coordinates.remove(unit_to_attack);

                    if units[*unit_to_attack].race == Elf {
                        elves_count -= 1;
                    } else {
                        goblins_count -= 1;
                    }
                } else {
                    units[*unit_to_attack].hit_points -= ATTACK_POWER;
                }

                continue;
            }

            let mut visited = HashSet::from([*current_unit_coordinates]);
            let mut nodes = VecDeque::new();

            let mut movement_candidates: Vec<(Coordinates, Coordinates, usize)> = Vec::new();

            for adjacent_tile in adjacent_tiles {
                if !occupied_caverns.contains_key(&adjacent_tile)
                    && map[adjacent_tile.y][adjacent_tile.x] == OpenCavern
                {
                    nodes.push_back(PathNode {
                        starting_coordinates: adjacent_tile,
                        current_coordinates: adjacent_tile,
                        path_length: 1,
                    });
                    visited.insert(adjacent_tile);
                }
            }

            while !nodes.is_empty() {
                let node = nodes.pop_front().unwrap();

                for adjacent_tile in [
                    (node.current_coordinates.x, node.current_coordinates.y - 1).into(),
                    (node.current_coordinates.x - 1, node.current_coordinates.y).into(),
                    (node.current_coordinates.x + 1, node.current_coordinates.y).into(),
                    (node.current_coordinates.x, node.current_coordinates.y + 1).into(),
                ] {
                    if let Some(adjacent_id) = occupied_caverns.get(&adjacent_tile)
                        && units[*adjacent_id].race != current_unit.race
                    {
                        movement_candidates.push((
                            node.starting_coordinates,
                            adjacent_tile,
                            node.path_length,
                        ));
                    }

                    if !occupied_caverns.contains_key(&adjacent_tile)
                        && map[adjacent_tile.y][adjacent_tile.x] == OpenCavern
                        && !visited.contains(&adjacent_tile)
                    {
                        nodes.push_back(PathNode {
                            starting_coordinates: node.starting_coordinates,
                            current_coordinates: adjacent_tile,
                            path_length: node.path_length + 1,
                        });
                        visited.insert(adjacent_tile);
                    }
                }
            }

            if !movement_candidates.is_empty() {
                let shortest_path = movement_candidates
                    .iter()
                    .min_by(|a, b| a.2.cmp(&b.2))
                    .unwrap()
                    .2;

                let destination = movement_candidates
                    .iter()
                    .filter(|(_start, _end, path_length)| *path_length == shortest_path)
                    .min_by(|a, b| a.1.cmp(&b.1))
                    .unwrap()
                    .1;

                let new_coordinates = movement_candidates
                    .iter()
                    .filter(|(_start, end, path_length)| {
                        *path_length == shortest_path && *end == destination
                    })
                    .min_by(|a, b| a.0.cmp(&b.0))
                    .unwrap()
                    .0;

                occupied_caverns.remove(current_unit_coordinates);
                occupied_caverns.insert(new_coordinates, *id);

                unit_coordinates.insert(*id, new_coordinates);

                let mut units_to_attack: Vec<usize> = Vec::with_capacity(4);

                let adjacent_tiles: [Coordinates; 4] = [
                    (new_coordinates.x, new_coordinates.y - 1).into(),
                    (new_coordinates.x - 1, new_coordinates.y).into(),
                    (new_coordinates.x + 1, new_coordinates.y).into(),
                    (new_coordinates.x, new_coordinates.y + 1).into(),
                ];

                for adjacent_tile in adjacent_tiles {
                    if let Some(id) = occupied_caverns.get(&adjacent_tile)
                        && units[*id].race != current_unit.race
                    {
                        units_to_attack.push(*id);
                    }
                }

                if !units_to_attack.is_empty() {
                    let min_health = units_to_attack
                        .iter()
                        .map(|id| units[*id].hit_points)
                        .min()
                        .unwrap();

                    let unit_to_attack = units_to_attack
                        .iter()
                        .find(|id| units[**id].hit_points == min_health)
                        .unwrap();

                    if units[*unit_to_attack].hit_points <= ATTACK_POWER {
                        units[*unit_to_attack].hit_points = 0;

                        let vacant_coordinates = unit_coordinates.get(unit_to_attack).unwrap();
                        occupied_caverns.remove(vacant_coordinates);
                        unit_coordinates.remove(unit_to_attack);

                        if units[*unit_to_attack].race == Elf {
                            elves_count -= 1;
                        } else {
                            goblins_count -= 1;
                        }
                    } else {
                        units[*unit_to_attack].hit_points -= ATTACK_POWER;
                    }
                }
            }
        }

        turn_order = unit_coordinates.keys().cloned().collect();
        turn_order.sort_unstable_by(|id1, id2| {
            unit_coordinates
                .get(id1)
                .unwrap()
                .cmp(unit_coordinates.get(id2).unwrap())
        });

        if elves_count == 0 || goblins_count == 0 {
            break;
        }

        full_rounds_count += 1;
    }

    dbg!(full_rounds_count);
    dbg!(&units);

    full_rounds_count * units.iter().map(|unit| unit.hit_points).sum::<usize>()
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
