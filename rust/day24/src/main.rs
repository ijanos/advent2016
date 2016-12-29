use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

type Pos = (u16, u16);

#[derive(Debug)]
struct Maze {
    map: HashSet<Pos>,
    goals: HashSet<Pos>,
    start: Pos,
}

fn read_maze(lines: Vec<String>) -> Maze {
    let mut map = HashSet::new();
    let mut goals = HashSet::new();
    let mut start = (0, 0);

    for (y, line) in lines.iter().enumerate() {
        for (x, title) in line.chars().enumerate() {
            let coord = (x as u16, y as u16);
            match title {
                '#' => (),
                '.' => {
                    map.insert(coord);
                }
                '0' => {
                    map.insert(coord);
                    start = coord;
                }
                n if n.is_digit(10) => {
                    map.insert(coord);
                    goals.insert(coord);
                }
                _ => unreachable!(),
            }
        }
    }

    Maze {
        map: map,
        goals: goals,
        start: start,
    }
}


fn find_path(start: Pos, goal: Pos, map: &HashSet<Pos>) -> u32 {
    let adjacent = |x, y| [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut state = vec![start];
    let mut steps = 0;
    loop {
        steps += 1;
        let mut new_state = Vec::with_capacity(state.len());
        for (x, y) in state {
            for n in &adjacent(x, y) {
                if goal == *n {
                    return steps;
                }
                if map.contains(n) && !visited.contains(n) {
                    visited.insert(*n);
                    new_state.push(*n);
                }
            }
        }
        state = new_state;
    }
}

fn solve(maze: &Maze) -> u32 {

    let mut steps = 0;
    let mut goals = maze.goals.clone();
    let mut start = maze.start;

    while !goals.is_empty() {
        let (nearest, s) = goals.iter()
            .cloned()
            .map(|g| (g, find_path(start, g, &maze.map)))
            .min_by_key(|&(_, n)| n)
            .unwrap();
        goals.remove(&nearest);
        start = nearest;
        steps += s;
    }

    steps
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();
    let maze = read_maze(lines);
    let part1 = solve(&maze);
    println!("part1: {}", part1);
}
