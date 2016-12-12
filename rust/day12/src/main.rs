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

// impl fmt::Display for Instruction {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let out = match self {
//             Cpy
//         }
//         write!(f, "{}", )
//     }
// }

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock().lines();

    let mut program = Vec::<Instruction>::new();

    for line in stdin {
        let line = line.unwrap();
        let instruction: Instruction = line.parse().unwrap();
        program.push(instruction);
    }

    let mut pc: isize = 0;
    let mut cpu = HashMap::<char, isize>::new();

    cpu.insert('a', 0);
    cpu.insert('b', 0);
    cpu.insert('c', 0);
    cpu.insert('d', 0);

    use Input::*;
    use Instruction::*;
    loop {
        match program[pc as usize] {
            Cpy(Value(x), reg) => {
                *cpu.get_mut(&reg).unwrap() = x;
                pc += 1;
            }
            Cpy(Register(reg1), reg2) => {
                *cpu.get_mut(&reg2).unwrap() = *cpu.get(&reg1).unwrap();
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
                if *cpu.get(&reg).unwrap() != 0 {
                    pc += offset
                } else {
                    pc += 1
                }
            }
        }

        if pc >= program.len() as isize {
            println!("part 1: {}", cpu[&'a']);
            break;
        }
    }
}
