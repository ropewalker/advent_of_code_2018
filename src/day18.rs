use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Acre {
    OpenGround,
    Trees,
    Lumberyard,
}

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<Vec<Acre>> {
    use Acre::*;

    let mut scan = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        let mut row = Vec::with_capacity(line.len());

        for char in line.chars() {
            row.push(match char {
                '.' => OpenGround,
                '|' => Trees,
                '#' => Lumberyard,
                _ => unreachable!(),
            })
        }

        scan.push(row);
    }

    scan
}

fn adjacent_acres(scan: &[Vec<Acre>], x: usize, y: usize) -> (usize, usize, usize) {
    use Acre::*;

    let (mut open_ground_count, mut trees_count, mut lumberyard_count) = (0, 0, 0);

    for adj_y in i32::max(y as i32 - 1, 0)..=i32::min(y as i32 + 1, scan.len() as i32 - 1) {
        for adj_x in i32::max(x as i32 - 1, 0)..=i32::min(x as i32 + 1, scan[0].len() as i32 - 1) {
            if adj_x as usize == x && adj_y as usize == y {
                continue;
            }

            match scan[adj_y as usize][adj_x as usize] {
                OpenGround => open_ground_count += 1,
                Trees => trees_count += 1,
                Lumberyard => lumberyard_count += 1,
            }
        }
    }

    (open_ground_count, trees_count, lumberyard_count)
}

fn resource_value_after(scan: &[Vec<Acre>], minutes: usize) -> usize {
    use Acre::*;

    let mut scan = scan.to_owned();
    let mut minute: usize = 0;

    let mut scans_cache = HashMap::from([(scan.clone(), minute)]);

    loop {
        minute += 1;

        let mut new_scan = Vec::with_capacity(scan.len());

        for y in 0..scan.len() {
            let mut new_scan_row = Vec::with_capacity(scan[0].len());

            for x in 0..scan[0].len() {
                let (_open_ground_count, trees_count, lumberyard_count) =
                    adjacent_acres(&scan, x, y);

                new_scan_row.push(match scan[y][x] {
                    OpenGround => {
                        if trees_count >= 3 {
                            Trees
                        } else {
                            OpenGround
                        }
                    }
                    Trees => {
                        if lumberyard_count >= 3 {
                            Lumberyard
                        } else {
                            Trees
                        }
                    }
                    Lumberyard => {
                        if lumberyard_count >= 1 && trees_count >= 1 {
                            Lumberyard
                        } else {
                            OpenGround
                        }
                    }
                });
            }

            new_scan.push(new_scan_row);
        }

        scan = new_scan;

        if minute == minutes {
            break;
        }

        if let Some(prev_minute) = scans_cache.get(&scan) {
            let delta_minutes = minute - *prev_minute;
            let jump = (minutes - minute) / delta_minutes;

            minute += jump * delta_minutes;
        }

        scans_cache.insert(scan.clone(), minute);
    }

    scan.iter().flatten().filter(|acre| **acre == Trees).count()
        * scan
            .iter()
            .flatten()
            .filter(|acre| **acre == Lumberyard)
            .count()
}

#[aoc(day18, part1)]
fn part1(scan: &[Vec<Acre>]) -> usize {
    resource_value_after(scan, 10)
}

#[aoc(day18, part2)]
fn part2(scan: &[Vec<Acre>]) -> usize {
    resource_value_after(scan, 1_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 1_147);
    }
}
