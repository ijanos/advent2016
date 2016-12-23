use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Input {
    Value(isize),
    Register(char),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Cpy(Input, Input),
    Inc(char),
    Dec(char),
    Jnz(Input, Input),
    Tgl(char),
}

impl FromStr for Instruction {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Instruction, Self::Err> {
        let input = input.split_whitespace().collect::<Vec<_>>();
        use Instruction::*;
        let to_char = |text: &str| text.chars().next().unwrap();
        let get_input = |text: &str| match text.parse::<isize>() {
            Ok(v) => Input::Value(v),
            Err(_) => Input::Register(to_char(text)),
        };
        match input[0] {
            "cpy" => Ok(Cpy(get_input(input[1]), get_input(input[2]))),
            "inc" => Ok(Inc(to_char(input[1]))),
            "dec" => Ok(Dec(to_char(input[1]))),
            "jnz" => Ok(Jnz(get_input(input[1]), get_input(input[2]))),
            "tgl" => Ok(Tgl(to_char(input[1]))),
            _ => Err("invalid instruction"),
        }
    }
}

fn toggle(ins: Instruction) -> Instruction {
    use Instruction::*;
    match ins {
        Inc(a) => Dec(a),
        Dec(a) | Tgl(a) => Inc(a),
        Jnz(a, b) => Cpy(a, b),
        Cpy(a, b) => Jnz(a, b),
    }
}

fn run(cpu: &mut HashMap<char, isize>, program: &mut Vec<Instruction>) -> isize {
    use Input::*;
    use Instruction::*;

    let mut pc: isize = 0;

    loop {
        let ins = program[pc as usize];
        match ins {
            Cpy(x, Register(y)) => {
                let x = match x {
                    Register(reg) => cpu[&reg],
                    Value(v) => v,
                };
                *cpu.get_mut(&y).unwrap() = x;
                pc += 1;
            }
            Cpy(_, Value(_)) => {
                pc += 1;
            }
            Inc(reg) => {
                *cpu.get_mut(&reg).unwrap() += 1;
                pc += 1;
            }
            Dec(reg) => {
                *cpu.get_mut(&reg).unwrap() -= 1;
                pc += 1;
            }
            Jnz(ref x, ref y) => {
                let x = match x {
                    &Register(reg) => cpu[&reg],
                    &Value(v) => v,
                };
                let offset = match y {
                    &Register(reg) => cpu[&reg],
                    &Value(v) => v,
                };

                if x != 0 { pc += offset } else { pc += 1 }
            }
            Tgl(reg) => {
                let offset = cpu[&reg];
                let addr = (pc + offset) as usize;
                if addr < program.len() {
                    program[addr] = toggle(program[addr]);
                }
                pc += 1;
            }
        }

        if pc >= program.len() as isize {
            return cpu[&'a'];
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock().lines();

    let mut program = Vec::<Instruction>::new();

    for line in stdin {
        let line = line.unwrap();
        let instruction: Instruction = line.parse().unwrap();
        program.push(instruction);
    }

    let mut cpu = HashMap::<char, isize>::new();

    cpu.insert('a', 7);
    cpu.insert('b', 0);
    cpu.insert('c', 0);
    cpu.insert('d', 0);
    let part1 = run(&mut cpu, &mut program);

    println!("part 1: {}", part1);
}
