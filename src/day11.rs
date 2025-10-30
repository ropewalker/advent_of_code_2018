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
}
