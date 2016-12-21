#![feature(advanced_slice_patterns, slice_patterns)]

extern crate permutohedron;

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;
use permutohedron::Heap;

fn scramble(input: &str, commands: &Vec<String>) -> String {
    let mut password: VecDeque<char> = input.chars().collect();
    for line in commands {
        let line = line.split_whitespace().collect::<Vec<_>>();
        match &line[..] {
            &["rotate", "right", x, _..] => {
                let x: usize = x.parse().unwrap();
                for _ in 0..x {
                    let tmp = password.pop_back().unwrap();
                    password.push_front(tmp);
                }
            }
            &["rotate", "left", x, _..] => {
                let x: usize = x.parse().unwrap();
                for _ in 0..x {
                    let tmp = password.pop_front().unwrap();
                    password.push_back(tmp);
                }
            }
            &["rotate", _.., x] => {
                let x = x.chars().nth(0).unwrap();
                let x = password.iter().position(|&c| c == x).unwrap();
                let rotate = if x >= 4 { x + 2 } else { x + 1 };
                for _ in 0..rotate {
                    let tmp = password.pop_back().unwrap();
                    password.push_front(tmp);
                }
            }
            &["reverse", _, x, _, y] => {
                let x: usize = x.parse().unwrap();
                let y: usize = y.parse().unwrap();
                let start = password.iter().take(x).cloned().collect::<Vec<char>>();
                let mut mid =
                    password.iter().skip(x).take(y - x + 1).cloned().collect::<Vec<char>>();
                mid.reverse();
                let end = password.iter().skip(y + 1).cloned().collect::<Vec<char>>();
                password.clear();
                password.extend(start.into_iter());
                password.extend(mid.into_iter());
                password.extend(end.into_iter());
            }
            &["move", _, x, _.., y] => {
                let x: usize = x.parse().unwrap();
                let y: usize = y.parse().unwrap();
                let c = password.remove(x).unwrap();
                password.insert(y, c);
            }
            &["swap", "letter", x, _.., y] => {
                let x = x.chars().nth(0).unwrap();
                let x = password.iter().position(|&c| c == x).unwrap();
                let y = y.chars().nth(0).unwrap();
                let y = password.iter().position(|&c| c == y).unwrap();
                password.swap(x, y);
            }
            &["swap", "position", x, _.., y] => {
                let x: usize = x.parse().unwrap();
                let y: usize = y.parse().unwrap();
                password.swap(x, y);
            }
            _ => unreachable!(),
        }
    }
    password.into_iter().collect()
}

fn main() {
    let stdin = io::stdin();
    let commands: Vec<String> = stdin.lock().lines().map(|line| line.unwrap()).collect();

    let part1 = scramble("abcdefgh", &commands);
    println!("part 1: {}", part1);

    let part2_scrambled = "fbgdceah";
    let mut letters: Vec<char> = part2_scrambled.chars().collect();
    let heap = Heap::new(&mut letters);
    for password in heap {
        let password: String = password.into_iter().collect();
        if scramble(&password, &commands) == part2_scrambled {
            println!("part 2: {}", password);
            break;
        }
    }
}
