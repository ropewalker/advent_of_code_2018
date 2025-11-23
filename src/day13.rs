use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
enum Tile {
    HorizontalPath,
    VerticalPath,
    PositiveCurve,
    NegativeCurve,
    Intersection,
    Empty,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn turn(&self, decision: Decision) -> Self {
        use Decision::*;
        use Direction::*;

        match decision {
            TurnLeft => match (*self as usize + 3) % 4 {
                0 => Right,
                1 => Down,
                2 => Left,
                3 => Up,
                _ => unreachable!(),
            },
            GoStraight => *self,
            TurnRight => match (*self as usize + 1) % 4 {
                0 => Right,
                1 => Down,
                2 => Left,
                3 => Up,
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Decision {
    TurnLeft = 0,
    GoStraight = 1,
    TurnRight = 2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
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

#[derive(Copy, Clone, Debug)]
struct Cart {
    direction: Direction,
    next_intersection: Decision,
}

impl From<(Direction, Decision)> for Cart {
    fn from(value: (Direction, Decision)) -> Self {
        Cart {
            direction: value.0,
            next_intersection: value.1,
        }
    }
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> (Vec<Vec<Tile>>, BTreeMap<Coordinates, Cart>) {
    use Decision::*;
    use Direction::*;
    use Tile::*;

    let mut map = Vec::with_capacity(input.lines().count());
    let mut carts = BTreeMap::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::with_capacity(line.len());

        for (x, char) in line.chars().enumerate() {
            match char {
                '|' => {
                    row.push(VerticalPath);
                }
                '-' => {
                    row.push(HorizontalPath);
                }
                '/' => {
                    row.push(NegativeCurve);
                }
                '\\' => {
                    row.push(PositiveCurve);
                }
                '+' => {
                    row.push(Intersection);
                }
                '^' => {
                    row.push(VerticalPath);
                    carts.insert((x, y).into(), (Up, TurnLeft).into());
                }
                'v' => {
                    row.push(VerticalPath);
                    carts.insert((x, y).into(), (Down, TurnLeft).into());
                }
                '<' => {
                    row.push(HorizontalPath);
                    carts.insert((x, y).into(), (Left, TurnLeft).into());
                }
                '>' => {
                    row.push(HorizontalPath);
                    carts.insert((x, y).into(), (Right, TurnLeft).into());
                }
                ' ' => {
                    row.push(Empty);
                }
                _ => unreachable!(),
            }
        }

        map.push(row);
    }

    (map, carts)
}

#[aoc(day13, part1)]
fn part1((map, carts): &(Vec<Vec<Tile>>, BTreeMap<Coordinates, Cart>)) -> Coordinates {
    use Decision::*;
    use Direction::*;
    use Tile::*;

    let mut carts = carts.clone();

    loop {
        let mut new_carts = carts.clone();

        for (coordinates, cart) in carts.iter() {
            let new_coordinates: Coordinates = match cart.direction {
                Up => (coordinates.x, coordinates.y - 1).into(),
                Down => (coordinates.x, coordinates.y + 1).into(),
                Left => (coordinates.x - 1, coordinates.y).into(),
                Right => (coordinates.x + 1, coordinates.y).into(),
            };

            new_carts.remove(coordinates);

            if new_carts.contains_key(&new_coordinates) {
                return new_coordinates;
            }

            let moved_cart = match map[new_coordinates.y][new_coordinates.x] {
                HorizontalPath | VerticalPath => *cart,
                PositiveCurve => match cart.direction {
                    Up => (Left, cart.next_intersection).into(),
                    Down => (Right, cart.next_intersection).into(),
                    Left => (Up, cart.next_intersection).into(),
                    Right => (Down, cart.next_intersection).into(),
                },
                NegativeCurve => match cart.direction {
                    Up => (Right, cart.next_intersection).into(),
                    Down => (Left, cart.next_intersection).into(),
                    Left => (Down, cart.next_intersection).into(),
                    Right => (Up, cart.next_intersection).into(),
                },
                Intersection => (
                    cart.direction.turn(cart.next_intersection),
                    match (cart.next_intersection as usize + 1) % 3 {
                        0 => TurnLeft,
                        1 => GoStraight,
                        2 => TurnRight,
                        _ => unreachable!(),
                    },
                )
                    .into(),
                _ => unreachable!(),
            };

            new_carts.insert(new_coordinates, moved_cart);
        }

        carts = new_carts;
    }
}

#[aoc(day13, part2)]
fn part2((map, carts): &(Vec<Vec<Tile>>, BTreeMap<Coordinates, Cart>)) -> Coordinates {
    use Decision::*;
    use Direction::*;
    use Tile::*;

    let mut carts = carts.clone();

    loop {
        if carts.len() == 1 {
            return *carts.keys().next().unwrap();
        }

        let mut new_carts = carts.clone();

        for (coordinates, cart) in carts.iter() {
            if !new_carts.contains_key(coordinates) {
                continue;
            }

            let new_coordinates: Coordinates = match cart.direction {
                Up => (coordinates.x, coordinates.y - 1).into(),
                Down => (coordinates.x, coordinates.y + 1).into(),
                Left => (coordinates.x - 1, coordinates.y).into(),
                Right => (coordinates.x + 1, coordinates.y).into(),
            };

            new_carts.remove(coordinates);

            if let std::collections::btree_map::Entry::Vacant(e) = new_carts.entry(new_coordinates)
            {
                let moved_cart = match map[new_coordinates.y][new_coordinates.x] {
                    HorizontalPath | VerticalPath => *cart,
                    PositiveCurve => match cart.direction {
                        Up => (Left, cart.next_intersection).into(),
                        Down => (Right, cart.next_intersection).into(),
                        Left => (Up, cart.next_intersection).into(),
                        Right => (Down, cart.next_intersection).into(),
                    },
                    NegativeCurve => match cart.direction {
                        Up => (Right, cart.next_intersection).into(),
                        Down => (Left, cart.next_intersection).into(),
                        Left => (Down, cart.next_intersection).into(),
                        Right => (Up, cart.next_intersection).into(),
                    },
                    Intersection => (
                        cart.direction.turn(cart.next_intersection),
                        match (cart.next_intersection as usize + 1) % 3 {
                            0 => TurnLeft,
                            1 => GoStraight,
                            2 => TurnRight,
                            _ => unreachable!(),
                        },
                    )
                        .into(),
                    _ => unreachable!(),
                };

                e.insert(moved_cart);
            } else {
                new_carts.remove(&new_coordinates);
            }
        }

        carts = new_carts;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";

    static TEST_INPUT_2: &str = r"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse_input(TEST_INPUT_1)),
            Coordinates { x: 7, y: 3 }
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse_input(TEST_INPUT_2)),
            Coordinates { x: 6, y: 4 }
        );
    }
}
