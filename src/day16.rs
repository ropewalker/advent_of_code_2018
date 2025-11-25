use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type Instruction = [usize; 4];
type MemoryState = [usize; 4];
type Program = Vec<Instruction>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Sample {
    before: MemoryState,
    instruction: Instruction,
    after: MemoryState,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> (Vec<Sample>, Program) {
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(
        sections(
            "Before: [" b0:usize ", " b1:usize ", " b2:usize ", " b3:usize "]\n"
            i0:usize " " i1:usize " " i2:usize " " i3:usize "\n"
            "After:  [" a0:usize ", " a1:usize ", " a2:usize ", " a3:usize "]\n"
                => Sample {
                    before: [b0, b1, b2, b3],
                    instruction: [i0, i1, i2, i3],
                    after: [a0, a1, a2, a3],
                }
        )
        "\n\n"
        section(
            lines(i0:usize " " i1:usize " " i2:usize " " i3:usize => [i0, i1, i2, i3])
        )
    );

    parser.parse(input).unwrap()
}

fn execute_instruction(
    memory_state: &MemoryState,
    opcode: &Opcode,
    instruction: &Instruction,
) -> MemoryState {
    use Opcode::*;

    let mut result = *memory_state;

    result[instruction[3]] = match opcode {
        Addr => result[instruction[1]] + result[instruction[2]],
        Addi => result[instruction[1]] + instruction[2],
        Mulr => result[instruction[1]] * result[instruction[2]],
        Muli => result[instruction[1]] * instruction[2],
        Banr => result[instruction[1]] & result[instruction[2]],
        Bani => result[instruction[1]] & instruction[2],
        Borr => result[instruction[1]] | result[instruction[2]],
        Bori => result[instruction[1]] | instruction[2],
        Setr => result[instruction[1]],
        Seti => instruction[1],
        Gtir => {
            if instruction[1] > result[instruction[2]] {
                1
            } else {
                0
            }
        }
        Gtri => {
            if result[instruction[1]] > instruction[2] {
                1
            } else {
                0
            }
        }
        Gtrr => {
            if result[instruction[1]] > result[instruction[2]] {
                1
            } else {
                0
            }
        }
        Eqir => {
            if instruction[1] == result[instruction[2]] {
                1
            } else {
                0
            }
        }
        Eqri => {
            if result[instruction[1]] == instruction[2] {
                1
            } else {
                0
            }
        }
        Eqrr => {
            if result[instruction[1]] == result[instruction[2]] {
                1
            } else {
                0
            }
        }
    };

    result
}

fn possible_opcodes(sample: &Sample) -> HashSet<Opcode> {
    use Opcode::*;

    [
        Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri,
        Eqrr,
    ]
    .into_iter()
    .filter(|opcode| {
        execute_instruction(&sample.before, opcode, &sample.instruction) == sample.after
    })
    .collect::<HashSet<_>>()
}

#[aoc(day16, part1)]
fn part1((samples, _program): &(Vec<Sample>, Program)) -> usize {
    samples
        .iter()
        .filter(|sample| possible_opcodes(sample).len() >= 3)
        .count()
}

#[aoc(day16, part2)]
fn part2((samples, program): &(Vec<Sample>, Program)) -> usize {
    let mut potential_opcodes = HashMap::with_capacity(16);

    for sample in samples.iter() {
        let mut possible_opcodes = possible_opcodes(sample);

        if let Some(known_opcodes) = potential_opcodes.get(&sample.instruction[0]) {
            possible_opcodes = possible_opcodes
                .intersection(known_opcodes)
                .cloned()
                .collect();
        }

        potential_opcodes.insert(sample.instruction[0], possible_opcodes);
    }

    let mut definite_opcodes = HashMap::with_capacity(16);

    while !potential_opcodes.is_empty() {
        let extracted: HashMap<_, _> = potential_opcodes
            .extract_if(|_, v| v.len() == 1)
            .map(|(k, v)| (k, v.into_iter().next().unwrap()))
            .collect();

        for possible_opcodes in potential_opcodes.values_mut() {
            for excluded_opcode in extracted.values() {
                possible_opcodes.remove(excluded_opcode);
            }
        }

        definite_opcodes.extend(extracted.into_iter());
    }

    let mut memory_state = [0, 0, 0, 0];

    for instruction in program.iter() {
        memory_state = execute_instruction(
            &memory_state,
            definite_opcodes.get(&instruction[0]).unwrap(),
            instruction,
        );
    }

    memory_state[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";

    #[test]
    fn part1_example() {
        use aoc_parse::{parser, prelude::*};

        let parser = parser!(
            "Before: [" b0:usize ", " b1:usize ", " b2:usize ", " b3:usize "]\n"
            i0:usize " " i1:usize " " i2:usize " " i3:usize "\n"
            "After:  [" a0:usize ", " a1:usize ", " a2:usize ", " a3:usize "]"
                => Sample {
                    before: [b0, b1, b2, b3],
                    instruction: [i0, i1, i2, i3],
                    after: [a0, a1, a2, a3],
                }
        );

        parser.parse(TEST_INPUT).unwrap();

        let sample = parser.parse(TEST_INPUT).unwrap();

        assert_eq!(possible_opcodes(&sample).len(), 3);
    }
}
