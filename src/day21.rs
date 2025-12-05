use aoc_runner_derive::{aoc, aoc_generator};
use Opcode::*;

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

#[aoc_generator(day21)]
fn parse_input(input: &str) -> Program {
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

fn execute_program(program: &Program, memory: &mut MemoryState, halt_at: Option<usize>) {
    let mut ip_value = 0;

    while ip_value < program.instructions.len() {
        if let Some(value) = halt_at
            && value == ip_value
        {
            break;
        }

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
}

//This relies heavily on the structure of the specific input. I am unsure if it would work for all possible inputs.
#[aoc(day21, part1)]
fn part1(program: &Program) -> usize {
    let mut memory: MemoryState = [0; 6];

    let halt_at = program
        .instructions
        .iter()
        .enumerate()
        .find_map(|(ip_value, instruction)| {
            if (instruction.1 == 0 || instruction.2 == 0) && instruction.0 == Eqrr {
                Some(ip_value)
            } else {
                None
            }
        });

    execute_program(program, &mut memory, halt_at);

    if let Some(ip_value) = halt_at {
        match program.instructions[ip_value] {
            (Eqrr, 0, x, _) | (Eqrr, x, 0, _) => return memory[x],
            _ => unreachable!(),
        }
    }

    memory[0]
}
