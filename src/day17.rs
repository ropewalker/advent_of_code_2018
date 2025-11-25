use crate::day17::VeinComponent::{Coordinate, Range};
use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::HashSet;

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

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Vec<ClayVein> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(
        {
            "x=" x:i32 ", y=" y_min:i32 ".." y_max:i32 => ClayVein {x: Coordinate(x),y: Range(y_min, y_max)},
            "y=" y:i32 ", x=" x_min:i32 ".." x_max:i32 => ClayVein {x: Range(x_min, x_max),y: Coordinate(y)},
        }
    ));

    parser.parse(input).unwrap()
}

#[aoc(day17, part1)]
fn part1(clay_veins: &[ClayVein]) -> usize {
    use VeinComponent::*;

    let mut clay_squares: HashSet<Coordinates> = HashSet::new();

    for clay_vein in clay_veins.iter() {
        if let Range(x_min, x_max) = clay_vein.x
            && let Coordinate(y) = clay_vein.y
        {
            for x in x_min..=x_max {
                clay_squares.insert((x, y).into());
            }
        } else if let Coordinate(x) = clay_vein.x
            && let Range(y_min, y_max) = clay_vein.y
        {
            for y in y_min..=y_max {
                clay_squares.insert((x, y).into());
            }
        }
    }

    unimplemented!()
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
}
