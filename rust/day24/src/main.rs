extern crate permutohedron;

use std::io;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

use permutohedron::Heap;

type Pos = (u16, u16);

#[derive(Debug)]
struct Maze {
    map: HashSet<Pos>,
    goals: Vec<Pos>,
    start: Pos,
}

fn read_maze(lines: Vec<String>) -> Maze {
    let mut map = HashSet::new();
    let mut goals = Vec::new();
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
                    goals.push(coord);
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

fn solve(maze: &Maze, part1: bool) -> u32 {
    let mut distances: HashMap<(Pos, Pos), u32> = HashMap::new();

    for (i, &g1) in maze.goals.iter().enumerate() {
        let d = find_path(maze.start, g1, &maze.map);
        distances.insert((maze.start, g1), d);
        distances.insert((g1, maze.start), d);
        for &g2 in &maze.goals[i..] {
            let d = find_path(g1, g2, &maze.map);
            distances.insert((g1, g2), d);
            distances.insert((g2, g1), d);
        }
    }

    let mut goals = maze.goals.clone();
    let heap = Heap::new(&mut goals);

    let mut path: Vec<Pos> = Vec::with_capacity(maze.goals.len() + 1);
    let mut min = u32::max_value();

    for mut g in heap {
        path.clear();
        path.push(maze.start);
        path.append(&mut g);
        if part1 {
            path.push(maze.start);
        }
        let cost = path.as_slice().windows(2).map(|w| distances[&(w[0], w[1])]).sum();
        if cost < min {
            min = cost
        }
    }
    min
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();
    let maze = read_maze(lines);
    let part1 = solve(&maze, false);
    let part2 = solve(&maze, true);
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
