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
    let mut summed_area = [[0; GRID_SIZE + 1]; GRID_SIZE + 1];

    for y in 1..=GRID_SIZE {
        for x in 1..=GRID_SIZE {
            summed_area[y][x] = power_level(x as i32, y as i32, *grid_serial_number)
                + summed_area[y - 1][x]
                + summed_area[y][x - 1]
                - summed_area[y - 1][x - 1];
        }
    }

    let mut max_total_power = summed_area[1][1];
    let (mut identifier_x, mut identifier_y, mut best_square_size) = (1, 1, 1);

    for square_size in 1..=GRID_SIZE {
        for y in square_size..=GRID_SIZE {
            for x in square_size..=GRID_SIZE {
                let total_power = summed_area[y][x]
                    - summed_area[y - square_size][x]
                    - summed_area[y][x - square_size]
                    + summed_area[y - square_size][x - square_size];
                if total_power > max_total_power {
                    max_total_power = total_power;
                    identifier_x = x;
                    identifier_y = y;
                    best_square_size = square_size;
                }
            }
        }
    }

    format!(
        "{},{},{}",
        identifier_x - best_square_size + 1,
        identifier_y - best_square_size + 1,
        best_square_size
    )
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
