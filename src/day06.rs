use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, VecDeque};

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<(i32, i32)> {
    use aoc_parse::{parser, prelude::*};
    let parser = parser!(lines(i32 ", " i32));
    parser.parse(input).unwrap()
}

fn manhattan_distance(coordinates1: &(i32, i32), coordinates2: &(i32, i32)) -> i32 {
    (coordinates1.0 - coordinates2.0).abs() + (coordinates1.1 - coordinates2.1).abs()
}

fn boundaries(coordinates: &[(i32, i32)]) -> (i32, i32, i32, i32) {
    coordinates.iter().fold(
        (
            coordinates[0].0,
            coordinates[0].0,
            coordinates[0].1,
            coordinates[0].1,
        ),
        |(x_min, x_max, y_min, y_max), (x, y)| {
            (
                i32::min(x_min, *x),
                i32::max(x_max, *x),
                i32::min(y_min, *y),
                i32::max(y_max, *y),
            )
        },
    )
}

#[aoc(day6, part1)]
fn part1(coordinates: &[(i32, i32)]) -> usize {
    let (x_min, x_max, y_min, y_max) = boundaries(coordinates);

    let mut areas = vec![Some(0usize); coordinates.len()];
    let mut visited: HashMap<(i32, i32), Option<usize>> =
        HashMap::with_capacity(((x_max - x_min) * (y_max - y_min)) as usize);
    let mut queue = coordinates
        .iter()
        .cloned()
        .enumerate()
        .collect::<VecDeque<_>>();

    while !queue.is_empty() {
        let (closest_coordinate_index, (x, y)) = queue.pop_front().unwrap();

        if visited.contains_key(&(x, y)) {
            if let Some(prev_closest_coordinate_index) = visited.get(&(x, y)).unwrap()
                && closest_coordinate_index != *prev_closest_coordinate_index
                && manhattan_distance(&(x, y), &coordinates[closest_coordinate_index])
                    == manhattan_distance(&(x, y), &coordinates[*prev_closest_coordinate_index])
            {
                if let Some(area) = areas[*prev_closest_coordinate_index] {
                    areas[*prev_closest_coordinate_index] = Some(area - 1);
                }

                visited.insert((x, y), None);
            }
        } else if x < x_min || x > x_max || y < y_min || y > y_max {
            areas[closest_coordinate_index] = None;
            visited.insert((x, y), Some(closest_coordinate_index));
        } else {
            if let Some(area) = areas[closest_coordinate_index] {
                areas[closest_coordinate_index] = Some(area + 1);
            }

            visited.insert((x, y), Some(closest_coordinate_index));

            for (x_new, y_new) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                queue.push_back((closest_coordinate_index, (x_new, y_new)));
            }
        }
    }

    areas.iter().max().unwrap().unwrap()
}

fn safe_region_area(coordinates: &[(i32, i32)], total_distance_threshold: i32) -> usize {
    let (x_min, x_max, y_min, y_max) = boundaries(coordinates);

    let mut area = 0;

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let total_distance = coordinates.iter().fold(0, |total_distance, coordinate| {
                total_distance + (coordinate.0 - x).abs() + (coordinate.1 - y).abs()
            });

            if total_distance < total_distance_threshold {
                area += 1;
            }
        }
    }

    area
}

#[aoc(day6, part2)]
fn part2(coordinates: &[(i32, i32)]) -> usize {
    safe_region_area(coordinates, 10_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 17);
    }

    #[test]
    fn part2_example() {
        assert_eq!(safe_region_area(&parse_input(TEST_INPUT), 32), 16);
    }
}
