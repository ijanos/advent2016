#![feature(advanced_slice_patterns, slice_patterns)]

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn main() {
    let mut part1: VecDeque<char> = "abcdefgh".chars().collect();


    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.split_whitespace().collect::<Vec<_>>();
        match &line[..] {
            &["rotate", "right", x, _..] => {
                let x: usize = x.parse().unwrap();
                for _ in 0..x {
                    let tmp = part1.pop_back().unwrap();
                    part1.push_front(tmp);
                }
            }
            &["rotate", "left", x, _..] => {
                let x: usize = x.parse().unwrap();
                for _ in 0..x {
                    let tmp = part1.pop_front().unwrap();
                    part1.push_back(tmp);
                }
            }
            &["rotate", _.., x] => {
                let x = x.chars().nth(0).unwrap();
                let x = part1.iter().position(|&c| c == x).unwrap();
                let rotate = if x >= 4 { x + 2 } else { x + 1 };
                for _ in 0..rotate {
                    let tmp = part1.pop_back().unwrap();
                    part1.push_front(tmp);
                }
            }
            &["reverse", _, x, _, y] => {
                let x: usize = x.parse().unwrap();
                let y: usize = y.parse().unwrap();
                let start = part1.iter().take(x).cloned().collect::<Vec<char>>();
                let mut mid = part1.iter().skip(x).take(y - x + 1).cloned().collect::<Vec<char>>();
                mid.reverse();
                let end = part1.iter().skip(y + 1).cloned().collect::<Vec<char>>();
                part1.clear();
                part1.extend(start.into_iter());
                part1.extend(mid.into_iter());
                part1.extend(end.into_iter());
            }
            &["move", _, x, _.., y] => {
                let x: usize = x.parse().unwrap();
                let y: usize = y.parse().unwrap();
                let c = part1.remove(x).unwrap();
                part1.insert(y, c);
            }
            &["swap", "letter", x, _.., y] => {
                let x = x.chars().nth(0).unwrap();
                let x = part1.iter().position(|&c| c == x).unwrap();
                let y = y.chars().nth(0).unwrap();
                let y = part1.iter().position(|&c| c == y).unwrap();
                part1.swap(x, y);
            }
            &["swap", "position", x, _.., y] => {
                let x: usize = x.parse().unwrap();
                let y: usize = y.parse().unwrap();
                part1.swap(x, y);
            }
            _ => unreachable!(),
        }
    }
    let part1: String = part1.into_iter().collect();
    println!("part 1: {}", part1);
}
