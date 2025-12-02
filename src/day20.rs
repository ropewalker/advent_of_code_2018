use aoc_runner_derive::aoc;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::hash::Hash;

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

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
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

fn match_regex(
    regex: &str,
    starting_token_index: usize,
    doors: &mut HashMap<Coordinates, HashSet<Coordinates>>,
    starting_rooms: &HashSet<Coordinates>,
) -> (HashSet<Coordinates>, usize) {
    let mut current_index = starting_token_index;
    let mut final_rooms: HashSet<Coordinates> = HashSet::new();
    let mut current_rooms = starting_rooms.clone();

    while current_index < regex.len() {
        let next_token = regex.chars().nth(current_index).unwrap();

        match next_token {
            '^' => current_index += 1,
            'N' | 'S' | 'E' | 'W' => {
                let mut next_rooms = HashSet::new();

                for room in current_rooms.iter() {
                    let next_room = match next_token {
                        'N' => (room.x, room.y - 1).into(),
                        'S' => (room.x, room.y + 1).into(),
                        'E' => (room.x + 1, room.y).into(),
                        'W' => (room.x - 1, room.y).into(),
                        _ => unreachable!(),
                    };

                    doors.entry(*room).or_default().insert(next_room);
                    doors.entry(next_room).or_default().insert(*room);

                    next_rooms.insert(next_room);
                }

                current_rooms = next_rooms;
                current_index += 1;
            }
            '(' => {
                let (rooms, index) = match_regex(regex, current_index + 1, doors, &current_rooms);

                current_rooms = rooms;
                current_index = index;
            }
            '|' => {
                final_rooms.extend(current_rooms.clone());
                current_rooms = starting_rooms.clone();
                current_index += 1;
            }
            ')' => {
                final_rooms.extend(current_rooms.clone());
                current_index += 1;

                return (final_rooms, current_index);
            }
            '$' => {
                return (final_rooms, current_index);
            }
            _ => unreachable!(),
        }
    }

    unreachable!()
}

struct State {
    coordinates: Coordinates,
    path_len: usize,
}

#[aoc(day20, part1)]
fn part1(regex: &str) -> usize {
    let mut doors = HashMap::new();

    match_regex(regex, 0, &mut doors, &HashSet::from([(0, 0).into()]));

    let mut visited: HashSet<Coordinates> = HashSet::from([(0, 0).into()]);
    let mut queue: VecDeque<State> = VecDeque::from([State {
        coordinates: (0, 0).into(),
        path_len: 0,
    }]);

    let mut longest_path = 0;

    while let Some(state) = queue.pop_front() {
        for adjacent_room in doors.get(&state.coordinates).unwrap_or(&HashSet::default()) {
            if !visited.contains(adjacent_room) {
                let new_path_len = state.path_len + 1;

                queue.push_back(State {
                    coordinates: *adjacent_room,
                    path_len: new_path_len,
                });
                visited.insert(*adjacent_room);

                if new_path_len > longest_path {
                    longest_path = new_path_len;
                }
            }
        }
    }

    longest_path
}

const LONG_PATH_LEN: usize = 1_000;

#[aoc(day20, part2)]
fn part2(regex: &str) -> usize {
    let mut doors = HashMap::new();

    match_regex(regex, 0, &mut doors, &HashSet::from([(0, 0).into()]));

    let mut visited: HashSet<Coordinates> = HashSet::from([(0, 0).into()]);
    let mut queue: VecDeque<State> = VecDeque::from([State {
        coordinates: (0, 0).into(),
        path_len: 0,
    }]);

    let mut far_rooms_count = 0;

    while let Some(state) = queue.pop_front() {
        for adjacent_room in doors.get(&state.coordinates).unwrap_or(&HashSet::default()) {
            if !visited.contains(adjacent_room) {
                let new_path_len = state.path_len + 1;

                queue.push_back(State {
                    coordinates: *adjacent_room,
                    path_len: new_path_len,
                });
                visited.insert(*adjacent_room);

                if new_path_len >= LONG_PATH_LEN {
                    far_rooms_count += 1;
                }
            }
        }
    }

    far_rooms_count
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = "^WNE$";
    static TEST_INPUT_2: &str = "^ENWWW(NEEE|SSE(EE|N))$";
    static TEST_INPUT_3: &str = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
    static TEST_INPUT_4: &str = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
    static TEST_INPUT_5: &str = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(TEST_INPUT_1), 3);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(TEST_INPUT_2), 10);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1(TEST_INPUT_3), 18);
    }

    #[test]
    fn part1_example4() {
        assert_eq!(part1(TEST_INPUT_4), 23);
    }

    #[test]
    fn part1_example5() {
        assert_eq!(part1(TEST_INPUT_5), 31);
    }
}
