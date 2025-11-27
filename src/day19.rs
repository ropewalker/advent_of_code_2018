use aoc_runner_derive::{aoc, aoc_generator};

type Instruction = (Opcode, usize, usize, usize);
type MemoryState = [usize; 6];

struct Program {
    ip_bound_to: usize,
    instructions: Vec<Instruction>,
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

#[aoc_generator(day19)]
fn parse_input(input: &str) -> Program {
    use Opcode::*;
    use aoc_parse::{parser, prelude::*};

    let parser = parser!(
        rule opcode: Opcode = {
            "addr" => Addr,
            "addi" => Addi,
            "mulr" => Mulr,
            "muli" => Muli,
            "banr" => Banr,
            "bani" => Bani,
            "borr" => Borr,
            "bori" => Bori,
            "setr" => Setr,
            "seti" => Seti,
            "gtir" => Gtir,
            "gtri" => Gtri,
            "gtrr" => Gtrr,
            "eqir" => Eqir,
            "eqri" => Eqri,
            "eqrr" => Eqrr,
        };

        ip_bound_to:line("#ip " usize)
        instructions:lines(opcode:opcode " " usize " " usize " " usize)
            => Program {ip_bound_to,instructions}
    );

    parser.parse(input).unwrap()
}

#[aoc(day19, part1)]
fn part1(program: &Program) -> usize {
    use Opcode::*;

    let mut memory: MemoryState = [0; 6];
    let mut ip_value = 0;

    while ip_value < program.instructions.len() {
        let instruction = &program.instructions[ip_value];

        memory[program.ip_bound_to] = ip_value;

        memory[instruction.3] = match instruction.0 {
            Addr => memory[instruction.1] + memory[instruction.2],
            Addi => memory[instruction.1] + instruction.2,
            Mulr => memory[instruction.1] * memory[instruction.2],
            Muli => memory[instruction.1] * instruction.2,
            Banr => memory[instruction.1] & memory[instruction.2],
            Bani => memory[instruction.1] & instruction.2,
            Borr => memory[instruction.1] | memory[instruction.2],
            Bori => memory[instruction.1] | instruction.2,
            Setr => memory[instruction.1],
            Seti => instruction.1,
            Gtir => {
                if instruction.1 > memory[instruction.2] {
                    1
                } else {
                    0
                }
            }
            Gtri => {
                if memory[instruction.1] > instruction.2 {
                    1
                } else {
                    0
                }
            }
            Gtrr => {
                if memory[instruction.1] > memory[instruction.2] {
                    1
                } else {
                    0
                }
            }
            Eqir => {
                if instruction.1 == memory[instruction.2] {
                    1
                } else {
                    0
                }
            }
            Eqri => {
                if memory[instruction.1] == instruction.2 {
                    1
                } else {
                    0
                }
            }
            Eqrr => {
                if memory[instruction.1] == memory[instruction.2] {
                    1
                } else {
                    0
                }
            }
        };

        ip_value = memory[program.ip_bound_to] + 1;
    }

    memory[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 6);
    }
}
