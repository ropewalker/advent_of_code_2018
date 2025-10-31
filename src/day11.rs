use aoc_runner_derive::{aoc, aoc_generator};

const GRID_SIZE: usize = 300;
const SQUARE_SIZE: usize = 3;

#[aoc_generator(day11)]
fn parse_input(input: &str) -> i32 {
    use aoc_parse::{parser, prelude::*};
    let parser = parser!(i32);
    parser.parse(input).unwrap()
}

#[aoc(day11, part1)]
fn part1(grid_serial_number: &i32) -> String {
    let mut grid = [[0; GRID_SIZE]; GRID_SIZE];

    for y in 1..=GRID_SIZE {
        for x in 1..=GRID_SIZE {
            grid[y - 1][x - 1] = power_level(x as i32, y as i32, *grid_serial_number);
        }
    }

    let mut max_total_power = grid[0][0]
        + grid[0][1]
        + grid[0][2]
        + grid[1][0]
        + grid[1][1]
        + grid[1][2]
        + grid[2][0]
        + grid[2][1]
        + grid[2][2];

    let mut largest_square = (1, 1);

    for y in 1..=GRID_SIZE - SQUARE_SIZE + 1 {
        for x in 1..=GRID_SIZE - SQUARE_SIZE + 1 {
            let total_power = grid[y - 1][x - 1]
                + grid[y - 1][x]
                + grid[y - 1][x + 1]
                + grid[y][x - 1]
                + grid[y][x]
                + grid[y][x + 1]
                + grid[y + 1][x - 1]
                + grid[y + 1][x]
                + grid[y + 1][x + 1];

            if total_power > max_total_power {
                max_total_power = total_power;
                largest_square = (x, y);
            }
        }
    }

    format!("{},{}", largest_square.0, largest_square.1)
}

fn power_level(x: i32, y: i32, grid_serial_number: i32) -> i32 {
    let rack_id = x + 10;

    (rack_id * y + grid_serial_number) * rack_id / 100 % 10 - 5
}

#[aoc(day11, part2)]
fn part2(grid_serial_number: &i32) -> String {
    let mut grid: Vec<Vec<Vec<i32>>> = Vec::with_capacity(GRID_SIZE);

    for y in 1..=GRID_SIZE {
        let mut total_powers_by_x = Vec::with_capacity(GRID_SIZE);

        for x in 1..=GRID_SIZE {
            total_powers_by_x.push(vec![power_level(x as i32, y as i32, *grid_serial_number)])
        }

        grid.push(total_powers_by_x);
    }

    let (x_minus_one, y_minus_one, mut max_total_power) = grid
        .iter()
        .enumerate()
        .flat_map(|(y, total_powers_by_x)| {
            total_powers_by_x.iter().enumerate().map(
                move |(x, total_powers_by_powers_of_two_square_size)| {
                    (x, y, total_powers_by_powers_of_two_square_size[0])
                },
            )
        })
        .max_by(|(_, _, power_level_1), (_, _, power_level_2)| power_level_1.cmp(power_level_2))
        .unwrap();

    let mut identifier = (x_minus_one + 1, y_minus_one + 1, 1);

    for square_size in 2..=GRID_SIZE {
        for y in 1..=GRID_SIZE {
            if y + square_size - 1 > GRID_SIZE {
                break;
            }

            for x in 1..=GRID_SIZE {
                if x + square_size - 1 > GRID_SIZE {
                    break;
                }

                if square_size.is_power_of_two() {
                    let total_power = grid[y - 1][x - 1][(square_size / 2).ilog2() as usize]
                        + grid[y - 1][x - 1 + square_size / 2][(square_size / 2).ilog2() as usize]
                        + grid[y - 1 + square_size / 2][x - 1][(square_size / 2).ilog2() as usize]
                        + grid[y - 1 + square_size / 2][x - 1 + square_size / 2]
                            [(square_size / 2).ilog2() as usize];

                    grid[y - 1][x - 1].push(total_power);

                    if total_power > max_total_power {
                        max_total_power = total_power;
                        identifier = (x, y, square_size);
                    }
                } else {
                    let mut covered_square_size = square_size.next_power_of_two() / 2;
                    let mut total_power = grid[y - 1][x - 1][covered_square_size.ilog2() as usize];

                    while covered_square_size < square_size {
                        let remainder = square_size - covered_square_size;

                        let previous_power_of_two = if remainder.is_power_of_two() {
                            remainder
                        } else {
                            remainder.next_power_of_two() / 2
                        };

                        let (mut smaller_square_x, mut smaller_square_y) =
                            (x + covered_square_size, y);

                        while smaller_square_y < y + covered_square_size {
                            total_power += grid[smaller_square_y - 1][smaller_square_x - 1]
                                [previous_power_of_two.ilog2() as usize];

                            smaller_square_y += previous_power_of_two;
                        }

                        (smaller_square_x, smaller_square_y) = (x, y + covered_square_size);

                        while smaller_square_x < x + covered_square_size {
                            total_power += grid[smaller_square_y - 1][smaller_square_x - 1]
                                [previous_power_of_two.ilog2() as usize];

                            smaller_square_x += previous_power_of_two;
                        }

                        (smaller_square_x, smaller_square_y) =
                            (x + covered_square_size, y + covered_square_size);

                        total_power += grid[smaller_square_y - 1][smaller_square_x - 1]
                            [previous_power_of_two.ilog2() as usize];

                        covered_square_size += previous_power_of_two;
                    }

                    if total_power > max_total_power {
                        max_total_power = total_power;
                        identifier = (x, y, square_size);
                    }
                }
            }
        }
    }

    format!("{},{},{}", identifier.0, identifier.1, identifier.2)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = "18";
    static TEST_INPUT_2: &str = "42";

    #[test]
    fn part1_example1() {
        assert_eq!(power_level(122, 79, 57), -5);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(power_level(217, 196, 39), 0);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(power_level(101, 153, 71), 4);
    }

    #[test]
    fn part1_example4() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), "33,45");
    }

    #[test]
    fn part1_example5() {
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), "21,61");
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&parse_input(TEST_INPUT_1)), "90,269,16");
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), "232,251,12");
    }
}
