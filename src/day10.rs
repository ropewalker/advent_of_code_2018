use aoc_runner_derive::{aoc, aoc_generator};

struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Point> {
    use aoc_parse::{parser, prelude::*};
    let parser = parser!(lines(
        "position=<" " "* x:i32 ", " " "* y:i32 "> velocity=<" " "* vx:i32 ", " " "* vy:i32 ">" =>
            Point {position: (x, y), velocity: (vx, vy)}
    ));
    parser.parse(input).unwrap()
}

fn message_and_time(points: &[Point]) -> (String, usize) {
    let mut positions: Vec<(i32, i32)> = points.iter().map(|point| point.position).collect();
    let velocities: Vec<(i32, i32)> = points.iter().map(|point| point.velocity).collect();

    let mut min_x = positions.iter().map(|position| position.0).min().unwrap();
    let mut max_x = positions.iter().map(|position| position.0).max().unwrap();
    let mut min_y = positions.iter().map(|position| position.1).min().unwrap();
    let mut max_y = positions.iter().map(|position| position.1).max().unwrap();

    let mut width = max_x - min_x + 1;
    let mut height = max_y - min_y + 1;

    let mut seconds_count = 0;

    loop {
        let new_positions: Vec<(i32, i32)> = positions
            .iter()
            .zip(velocities.iter())
            .map(|(position, velocity)| (position.0 + velocity.0, position.1 + velocity.1))
            .collect();

        let new_min_x = new_positions
            .iter()
            .map(|position| position.0)
            .min()
            .unwrap();
        let new_max_x = new_positions
            .iter()
            .map(|position| position.0)
            .max()
            .unwrap();
        let new_min_y = new_positions
            .iter()
            .map(|position| position.1)
            .min()
            .unwrap();
        let new_max_y = new_positions
            .iter()
            .map(|position| position.1)
            .max()
            .unwrap();

        let new_width = new_max_x - new_min_x + 1;
        let new_height = new_max_y - new_min_y + 1;

        if new_width >= width && new_height >= height {
            let mut message = String::new();

            for y in min_y..=max_y {
                message.push('\n');

                for x in min_x..=max_x {
                    if positions.contains(&(x, y)) {
                        message.push('#');
                    } else {
                        message.push('.');
                    }
                }
            }

            return (message, seconds_count);
        } else {
            width = new_width;
            height = new_height;
            positions = new_positions;
            min_x = new_min_x;
            max_x = new_max_x;
            min_y = new_min_y;
            max_y = new_max_y;
            seconds_count += 1;
        }
    }
}

#[aoc(day10, part1)]
fn part1(points: &[Point]) -> String {
    message_and_time(points).0
}

#[aoc(day10, part2)]
fn part2(points: &[Point]) -> usize {
    message_and_time(points).1
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse_input(TEST_INPUT_1)),
            r"
#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###"
        );
    }
}
