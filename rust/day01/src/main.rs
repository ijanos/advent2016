use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

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
    direction: Direction,
    visited: HashSet<(isize, isize)>,
    crossed: Vec<(isize, isize)>,
}

impl Direction {
    fn turn(&self, dir: char) -> Direction {
        use Direction::*;
        let left_right = |left, right| if dir == 'L' { left } else { right };
        match self {
            &North => left_right(West, East),
            &East => left_right(North, South),
            &South => left_right(East, West),
            &West => left_right(South, North),
        }
    }
}

impl State {
    fn new() -> State {
        State {
            x: 0,
            y: 0,
            direction: Direction::North,
            visited: HashSet::new(),
            crossed: Vec::new(),
        }
    }

    fn move_to(&mut self, turn: char, length: u32) {
        use Direction::*;
        self.direction = self.direction.turn(turn);
        for _ in 0..length {
            match self.direction {
                North => self.y += 1,
                East => self.x += 1,
                South => self.y -= 1,
                West => self.x -= 1,
            }
            if !self.visited.insert((self.x, self.y)) {
                self.crossed.push((self.x, self.y));
            }
        }
    }
}

fn parse(instruction: &str) -> (char, u32) {
    let mut chars = instruction.chars();
    let dir = chars.next().unwrap();
    let length = chars.collect::<String>().parse::<u32>().unwrap();
    (dir, length)
}


fn main() {
    let stdin = io::stdin();
    let mut position = State::new();
    // input only contains one line
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let instructions: Vec<&str> = line.split(", ").collect();
    for instruction in instructions {
        let (turn, n) = parse(instruction);
        position.move_to(turn, n);
    }
    let part1 = position.x.abs() + position.y.abs();
    println!("part 1: {:?}", part1);

    let &(x, y) = position.crossed.first().unwrap();
    println!("part 2: {:?}", x.abs() + y.abs());
}
