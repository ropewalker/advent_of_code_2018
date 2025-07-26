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

#[derive(Default)]
struct Intervals {
    edges: BTreeSet<usize>,
    from_edges: BTreeMap<usize, HashSet<usize>>,
    to_edges: BTreeMap<usize, HashSet<usize>>,
}

fn calculate_horizontal_intervals(claims: &HashMap<usize, Claim>) -> Intervals {
    claims.iter().fold(
        Intervals::default(),
        |mut horizontal_intervals, (id, claim)| {
            horizontal_intervals.edges.insert(claim.left);
            horizontal_intervals.edges.insert(claim.left + claim.width);

            (*horizontal_intervals
                .from_edges
                .entry(claim.left)
                .or_default())
            .insert(*id);

            (*horizontal_intervals
                .to_edges
                .entry(claim.left + claim.width)
                .or_default())
            .insert(*id);

            horizontal_intervals
        },
    )
}

fn update_vertical_intervals(
    claims: &HashMap<usize, Claim>,
    x: usize,
    vertical_intervals: &mut Intervals,
    horizontal_intervals: &Intervals,
) {
    for claim_id in horizontal_intervals
        .from_edges
        .get(&x)
        .into_iter()
        .flatten()
    {
        let claim = claims.get(claim_id).unwrap();

        (*vertical_intervals.from_edges.entry(claim.top).or_default()).insert(*claim_id);
        (*vertical_intervals
            .to_edges
            .entry(claim.top + claim.height)
            .or_default())
        .insert(*claim_id);

        vertical_intervals.edges.insert(claim.top);
        vertical_intervals.edges.insert(claim.top + claim.height);
    }

    for claim_id in horizontal_intervals.to_edges.get(&x).into_iter().flatten() {
        let claim = claims.get(claim_id).unwrap();

        let top_edges = vertical_intervals.from_edges.get_mut(&claim.top).unwrap();
        top_edges.remove(claim_id);

        if top_edges.is_empty() {
            vertical_intervals.from_edges.remove(&claim.top);

            if !vertical_intervals.to_edges.contains_key(&claim.top) {
                vertical_intervals.edges.remove(&claim.top);
            }
        }

        let bottom_edges = vertical_intervals
            .to_edges
            .get_mut(&(claim.top + claim.height))
            .unwrap();
        bottom_edges.remove(claim_id);

        if bottom_edges.is_empty() {
            vertical_intervals
                .to_edges
                .remove(&(claim.top + claim.height));

            if !vertical_intervals
                .from_edges
                .contains_key(&(claim.top + claim.height))
            {
                vertical_intervals.edges.remove(&(claim.top + claim.height));
            }
        }
    }
}

#[aoc(day3, part1)]
fn part1(claims: &HashMap<usize, Claim>) -> usize {
    let horizontal_intervals = calculate_horizontal_intervals(claims);
    let mut vertical_intervals = Intervals::default();

    let mut overlap_height = 0usize;
    let mut previous_x = 0usize;

    let mut result = 0;

    for x in horizontal_intervals.edges.iter() {
        result += overlap_height * (x - previous_x);
        previous_x = *x;

        update_vertical_intervals(claims, *x, &mut vertical_intervals, &horizontal_intervals);

        overlap_height = 0;
        let mut claim_count = 0;
        let mut previous_y = 0;

        for y in vertical_intervals.edges.iter() {
            if claim_count > 1 {
                overlap_height += y - previous_y;
            }

            previous_y = *y;

            if let Some(top_edges) = vertical_intervals.from_edges.get(y) {
                claim_count += top_edges.len();
            }

            if let Some(bottom_edges) = vertical_intervals.to_edges.get(y) {
                claim_count -= bottom_edges.len();
            }
        }
    }

    result
}

#[aoc(day3, part2)]
fn part2(claims: &HashMap<usize, Claim>) -> usize {
    let mut non_overlapped_claims_ids: HashSet<_> = claims.keys().cloned().collect();

    let horizontal_intervals = calculate_horizontal_intervals(claims);
    let mut vertical_intervals = Intervals::default();

    for x in horizontal_intervals.edges.iter() {
        update_vertical_intervals(claims, *x, &mut vertical_intervals, &horizontal_intervals);

        let mut current_claims_ids = HashSet::new();

        for y in vertical_intervals.edges.iter() {
            if let Some(top_edges) = vertical_intervals.from_edges.get(y) {
                for id in top_edges {
                    current_claims_ids.insert(*id);
                }
            }

            if let Some(bottom_edges) = vertical_intervals.to_edges.get(y) {
                for id in bottom_edges {
                    current_claims_ids.remove(id);
                }
            }

            if current_claims_ids.len() > 1 {
                for id in current_claims_ids.iter() {
                    non_overlapped_claims_ids.remove(id);
                }
            }
        }
    }

    non_overlapped_claims_ids.into_iter().next().unwrap()
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
