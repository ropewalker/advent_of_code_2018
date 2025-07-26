use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[derive(Copy, Clone, Debug)]
struct Claim {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> HashMap<usize, Claim> {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(lines(
        "#" id:usize
        " @ "
        left:usize "," top:usize
        ": "
        width:usize "x" height:usize =>
            (
                id,
                Claim {
                    left,
                    top,
                    width,
                    height,
                }
            )
    ));

    parser.parse(input).unwrap().into_iter().collect()
}

#[aoc(day3, part1)]
fn part1(claims: &HashMap<usize, Claim>) -> usize {
    let (x_coordinates, left_edges, right_edges) = claims.iter().fold(
        (BTreeSet::new(), BTreeMap::new(), BTreeMap::new()),
        |(mut x_coordinates, mut left_edges, mut right_edges), (id, claim)| {
            x_coordinates.insert(claim.left);
            x_coordinates.insert(claim.left + claim.width);

            (*left_edges.entry(claim.left).or_insert(HashSet::new())).insert(*id);

            (*right_edges
                .entry(claim.left + claim.width)
                .or_insert(HashSet::new()))
            .insert(*id);

            (x_coordinates, left_edges, right_edges)
        },
    );

    let mut top_edges: BTreeMap<usize, HashSet<usize>> = BTreeMap::new();
    let mut bottom_edges: BTreeMap<usize, HashSet<usize>> = BTreeMap::new();
    let mut y_coordinates: BTreeSet<usize> = BTreeSet::new();

    let mut overlap_height = 0usize;
    let mut previous_x = 0usize;

    let mut result = 0;

    for x in x_coordinates {
        result += overlap_height * (x - previous_x);
        previous_x = x;

        for claim_id in left_edges.get(&x).into_iter().flatten() {
            let claim = claims.get(claim_id).unwrap();

            (*top_edges.entry(claim.top).or_default()).insert(*claim_id);
            (*bottom_edges.entry(claim.top + claim.height).or_default()).insert(*claim_id);

            y_coordinates.insert(claim.top);
            y_coordinates.insert(claim.top + claim.height);
        }

        for claim_id in right_edges.get(&x).into_iter().flatten() {
            let claim = claims.get(claim_id).unwrap();

            let top_edges_at_y = top_edges.get_mut(&claim.top).unwrap();
            top_edges_at_y.remove(claim_id);

            if top_edges_at_y.is_empty() {
                top_edges.remove(&claim.top);

                if !bottom_edges.contains_key(&claim.top) {
                    y_coordinates.remove(&claim.top);
                }
            }

            let bottom_edges_at_y = bottom_edges.get_mut(&(claim.top + claim.height)).unwrap();
            bottom_edges_at_y.remove(claim_id);

            if bottom_edges_at_y.is_empty() {
                bottom_edges.remove(&(claim.top + claim.height));

                if !top_edges.contains_key(&(claim.top + claim.height)) {
                    y_coordinates.remove(&(claim.top + claim.height));
                }
            }
        }

        overlap_height = 0;
        let mut claim_count = 0;
        let mut previous_y = 0;

        for y in y_coordinates.iter() {
            if claim_count > 1 {
                overlap_height += y - previous_y;
            }

            previous_y = *y;

            if let Some(top_edges_at_y) = top_edges.get(y) {
                claim_count += top_edges_at_y.len();
            }

            if let Some(bottom_edges_at_y) = bottom_edges.get(y) {
                claim_count -= bottom_edges_at_y.len();
            }
        }
    }

    result
}

#[aoc(day3, part2)]
fn part2(claims: &HashMap<usize, Claim>) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 4);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 3);
    }
}
