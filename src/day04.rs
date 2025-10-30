use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
struct Timestamp {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year
            .cmp(&other.year)
            .then(self.month.cmp(&other.month))
            .then(self.day.cmp(&other.day))
            .then(self.hour.cmp(&other.hour))
            .then(self.minute.cmp(&other.minute))
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
enum Event {
    GuardBeginsShift(usize),
    FallsAsleep,
    WakesUp,
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<(Timestamp, Event)> {
    use Event::*;
    use aoc_parse::{parser, prelude::*};

    let timestamp = parser!(
        "[" year:usize "-" month:usize "-" day:usize " " hour:usize ":" minute:usize "] " =>
            Timestamp {
                year,
                month,
                day,
                hour,
                minute
            }
    );

    let event = parser!({
            "Guard #" id:usize " begins shift" => GuardBeginsShift(id),
            "falls asleep" => FallsAsleep,
            "wakes up" => WakesUp,
    });

    let parser = parser!(lines(timestamp event));
    parser.parse(input).unwrap()
}

fn minutes_per_guard(events: &[(Timestamp, Event)]) -> HashMap<usize, [usize; 60]> {
    use Event::*;
    let events = events.iter().cloned().collect::<BTreeMap<_, _>>();

    let mut minutes_per_guard: HashMap<usize, [usize; 60]> = HashMap::new();

    let mut current_id = match events.iter().next().unwrap().1 {
        GuardBeginsShift(id) => *id,
        _ => unreachable!(),
    };
    minutes_per_guard.insert(current_id, [0; 60]);
    let mut sleep_start = 0;

    for (timestamp, event) in events.iter().skip(1) {
        match event {
            GuardBeginsShift(id) => {
                current_id = *id;
                minutes_per_guard.entry(current_id).or_insert([0; 60]);
            }
            FallsAsleep => {
                sleep_start = timestamp.minute;
            }
            WakesUp => {
                for minute in sleep_start..timestamp.minute {
                    minutes_per_guard.get_mut(&current_id).unwrap()[minute] += 1;
                }
            }
        }
    }

    minutes_per_guard
}

#[aoc(day4, part1)]
fn part1(events: &[(Timestamp, Event)]) -> usize {
    let minutes_per_guard = minutes_per_guard(events);

    let (id, minutes) = minutes_per_guard
        .iter()
        .max_by(|(_, minutes_1), (_, minutes_2)| {
            minutes_1.iter().sum::<usize>().cmp(&minutes_2.iter().sum())
        })
        .unwrap();

    *id * minutes
        .iter()
        .enumerate()
        .max_by(|(_, count_1), (_, count_2)| count_1.cmp(count_2))
        .unwrap()
        .0
}

#[aoc(day4, part2)]
fn part2(events: &[(Timestamp, Event)]) -> usize {
    let minutes_per_guard = minutes_per_guard(events);

    let (id, minutes) = minutes_per_guard
        .iter()
        .max_by(|(_, minutes_1), (_, minutes_2)| {
            minutes_1.iter().max().cmp(&minutes_2.iter().max())
        })
        .unwrap();

    *id * minutes
        .iter()
        .enumerate()
        .max_by(|(_, count_1), (_, count_2)| count_1.cmp(count_2))
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 240);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 4_455);
    }
}
