use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Debug)]
struct State {
    x: isize,
    y: isize,
    direction: Direction
}

fn parse(instruction: &str) -> (char, u32) {
    let mut chars = instruction.chars();
    let dir = chars.next().unwrap();
    let length = chars.collect::<String>().parse::<u32>().unwrap();
    (dir, length)
}

fn go(state: State, turn: char, length: u32) -> State {
    use Direction::*;
    let length: isize = length as isize;
    let new_dir = if turn == 'R' {
        match state.direction {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    } else {
        match state.direction {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    };
    match new_dir {
        North => State{x: state.x, y: state.y + length, direction: new_dir },
        East =>  State{x: state.x + length, y: state.y, direction: new_dir },
        South => State{x: state.x, y: state.y - length, direction: new_dir },
        West => State{x: state.x - length, y: state.y, direction: new_dir },
    }
}

fn part1(instructions: &str) -> isize {
    let mut bunnies = State{x: 0, y: 0, direction: Direction::North};
    let instructions: Vec<&str> = instructions.split(", ").collect();
    for instruction in instructions {
        let (turn, n) = parse(instruction);
        bunnies = go(bunnies, turn, n);
    }
    bunnies.x.abs() + bunnies.y.abs()
}


fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        println!("{:?}", part1(&line));
    }
}
