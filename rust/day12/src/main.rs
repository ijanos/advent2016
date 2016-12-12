use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
enum Input {
    Value(isize),
    Register(char),
}

#[derive(Debug)]
enum Instruction {
    Cpy(Input, char),
    Inc(char),
    Dec(char),
    Jnz(Input, isize),
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
            "cpy" => Ok(Cpy(get_input(input[1]), to_char(input[2]))),
            "inc" => Ok(Inc(to_char(input[1]))),
            "dec" => Ok(Dec(to_char(input[1]))),
            "jnz" => Ok(Jnz(get_input(input[1]), input[2].parse::<isize>().unwrap())),
            _ => Err("invalid instruction"),
        }
    }
}

fn run(cpu: &mut HashMap<char, isize>, program: &Vec<Instruction>) -> isize {
    use Input::*;
    use Instruction::*;

    let mut pc: isize = 0;

    loop {
        match program[pc as usize] {
            Cpy(Value(x), reg) => {
                *cpu.get_mut(&reg).unwrap() = x;
                pc += 1;
            }
            Cpy(Register(reg1), reg2) => {
                *cpu.get_mut(&reg2).unwrap() = cpu[&reg1];
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
            Jnz(Value(x), offset) => if x != 0 { pc += offset } else { pc += 1 },
            Jnz(Register(reg), offset) => {
                if cpu[&reg] != 0 {
                    pc += offset
                } else {
                    pc += 1
                }
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
    let program = program;

    let mut cpu = HashMap::<char, isize>::new();

    cpu.insert('a', 0);
    cpu.insert('b', 0);
    cpu.insert('c', 0);
    cpu.insert('d', 0);
    let part1 = run(&mut cpu, &program);

    cpu.insert('a', 0);
    cpu.insert('b', 0);
    cpu.insert('c', 1);
    cpu.insert('d', 0);
    let part2 = run(&mut cpu, &program);

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
