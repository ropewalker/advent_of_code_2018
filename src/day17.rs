use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum VeinComponent {
    Coordinate(i32),
    Range(i32, i32),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct ClayVein {
    x: VeinComponent,
    y: VeinComponent,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
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

impl From<(i32, i32)> for Coordinates {
    fn from(value: (i32, i32)) -> Self {
        Coordinates {
            x: value.0,
            y: value.1,
        }
    }
}

const SPRING_COORDINATES: Coordinates = Coordinates { x: 500, y: 0 };

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Vec<ClayVein> {
    use VeinComponent::*;
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(
        {
            "x=" x:i32 ", y=" y_min:i32 ".." y_max:i32 => ClayVein {x: Coordinate(x),y: Range(y_min, y_max)},
            "y=" y:i32 ", x=" x_min:i32 ".." x_max:i32 => ClayVein {x: Range(x_min, x_max),y: Coordinate(y)},
        }
    ));

    parser.parse(input).unwrap()
}

fn is_clay(
    square: &Coordinates,
    vertical_veins: &HashMap<i32, Vec<(i32, i32)>>,
    horizontal_veins: &HashMap<i32, Vec<(i32, i32)>>,
) -> bool {
    if let Some(veins) = vertical_veins.get(&square.x) {
        for range in veins.iter() {
            if square.y >= range.0 && square.y <= range.1 {
                return true;
            }
        }
    }

    if let Some(veins) = horizontal_veins.get(&square.y) {
        for range in veins.iter() {
            if square.x >= range.0 && square.x <= range.1 {
                return true;
            }
        }
    }

    false
}

fn count_water_squares(
    starting_square: Coordinates,
    min_y: i32,
    max_y: i32,
    vertical_veins: &HashMap<i32, Vec<(i32, i32)>>,
    horizontal_veins: &mut HashMap<i32, Vec<(i32, i32)>>,
    dry: bool,
) -> usize {
    let mut starting_points = vec![starting_square];
    let mut reached = HashSet::new();

    let mut still_water: HashSet<Coordinates> = HashSet::new();

    'main_loop: while let Some(mut current_square) = starting_points.pop() {
        if current_square.y >= min_y && current_square.y <= max_y {
            if !reached.contains(&current_square) {
                reached.insert(current_square);
            } else {
                continue;
            }
        }

        let mut next_square = (current_square.x, current_square.y + 1).into();

        while !is_clay(&next_square, &vertical_veins, &horizontal_veins) {
            current_square = next_square;

            if current_square.y > max_y {
                continue 'main_loop;
            }

            if current_square.y >= min_y {
                if !reached.contains(&current_square) {
                    reached.insert(current_square);
                } else {
                    continue 'main_loop;
                }
            }

            next_square = (current_square.x, current_square.y + 1).into();
        }

        let mut left_wall = true;
        let mut right_wall = true;

        loop {
            let mut water_layer_left = current_square.x;
            let mut water_layer_right = current_square.x;
            reached.insert(current_square);

            let mut left_square: Coordinates = (current_square.x - 1, current_square.y).into();
            let mut below_left_square = (left_square.x, left_square.y + 1).into();

            while !is_clay(&left_square, vertical_veins, horizontal_veins) {
                if !is_clay(&below_left_square, vertical_veins, horizontal_veins) {
                    starting_points.push(left_square);

                    left_wall = false;
                    break;
                } else {
                    reached.insert(left_square);

                    water_layer_left = left_square.x;

                    left_square = (left_square.x - 1, left_square.y).into();
                    below_left_square = (left_square.x, left_square.y + 1).into();
                }
            }

            let mut right_square: Coordinates = (current_square.x + 1, current_square.y).into();
            let mut below_right_square = (right_square.x, right_square.y + 1).into();

            while !is_clay(&right_square, vertical_veins, horizontal_veins) {
                if !is_clay(&below_right_square, vertical_veins, horizontal_veins) {
                    starting_points.push(right_square);

                    right_wall = false;
                    break;
                } else {
                    reached.insert(right_square);

                    water_layer_right = right_square.x;

                    right_square = (right_square.x + 1, right_square.y).into();
                    below_right_square = (right_square.x, right_square.y + 1).into();
                }
            }

            if left_wall && right_wall {
                horizontal_veins
                    .entry(current_square.y)
                    .or_default()
                    .push((water_layer_left, water_layer_right));

                for x in water_layer_left..=water_layer_right {
                    still_water.insert((x, current_square.y).into());
                }

                current_square = (current_square.x, current_square.y - 1).into();
            } else {
                break;
            }
        }
    }

    if dry {
        still_water.len()
    } else {
        reached.len()
    }
}

struct ClayVeinsMap {
    vertical_veins: HashMap<i32, Vec<(i32, i32)>>,
    horizontal_veins: HashMap<i32, Vec<(i32, i32)>>,
    min_y: i32,
    max_y: i32,
}

fn init_map(clay_veins: &[ClayVein]) -> ClayVeinsMap {
    use VeinComponent::*;

    let mut vertical_veins = HashMap::new();
    let mut horizontal_veins = HashMap::new();

    for clay_vein in clay_veins.iter() {
        if let Range(x_min, x_max) = clay_vein.x
            && let Coordinate(y) = clay_vein.y
        {
            horizontal_veins
                .entry(y)
                .or_insert(Vec::new())
                .push((x_min, x_max));
        } else if let Coordinate(x) = clay_vein.x
            && let Range(y_min, y_max) = clay_vein.y
        {
            vertical_veins
                .entry(x)
                .or_insert(Vec::new())
                .push((y_min, y_max));
        }
    }

    let min_y = clay_veins
        .iter()
        .map(|vein| match vein.y {
            Coordinate(y) => y,
            Range(y_min, _) => y_min,
        })
        .min()
        .unwrap();
    let max_y = clay_veins
        .iter()
        .map(|vein| match vein.y {
            Coordinate(y) => y,
            Range(_, y_max) => y_max,
        })
        .max()
        .unwrap();

    ClayVeinsMap {
        horizontal_veins,
        vertical_veins,
        min_y,
        max_y,
    }
}

#[aoc(day17, part1)]
fn part1(clay_veins: &[ClayVein]) -> usize {
    let mut clay_veins_map = init_map(clay_veins);

    count_water_squares(
        SPRING_COORDINATES,
        clay_veins_map.min_y,
        clay_veins_map.max_y,
        &clay_veins_map.vertical_veins,
        &mut clay_veins_map.horizontal_veins,
        false,
    )
}

#[aoc(day17, part2)]
fn part2(clay_veins: &[ClayVein]) -> usize {
    let mut clay_veins_map = init_map(clay_veins);

    count_water_squares(
        SPRING_COORDINATES,
        clay_veins_map.min_y,
        clay_veins_map.max_y,
        &clay_veins_map.vertical_veins,
        &mut clay_veins_map.horizontal_veins,
        true,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 57);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 29);
    }
}
