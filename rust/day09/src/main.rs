use std::io;
use std::io::prelude::*;

fn parse_command(text: &[u8]) -> (usize, usize) {
    let text = std::str::from_utf8(&text[1..text.len() - 1]).unwrap();
    let command: Vec<_> = text.split('x').map(|n| n.parse::<usize>().unwrap()).collect();
    (command[0], command[1])
}

fn count(text: &[u8], part1: bool) -> usize {
    if text.contains(&b'(') {
        let open = text.iter().position(|&c| c == b'(').unwrap();
        let (front, tail) = text.split_at(open);
        let close = tail.iter().position(|&c| c == b')').unwrap();
        let (command, tail) = tail.split_at(close + 1);
        let (size, repeats) = parse_command(command);
        let (repeated, tail) = tail.split_at(size);
        if part1 {
            front.len() + repeats * size + count(tail, part1)
        } else {
            front.len() + repeats * count(repeated, part1) + count(tail, part1)
        }
    } else {
        text.len()
    }
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;

    let stdin = io::stdin();
    let stdin = stdin.lock().lines();
    for line in stdin {
        let line = line.unwrap();
        part1 += count(line.as_bytes(), true);
        part2 += count(line.as_bytes(), false);
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
