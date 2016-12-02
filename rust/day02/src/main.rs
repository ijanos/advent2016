use std::io;
use std::io::prelude::*;
use std::char;

fn part1_next(n: u32, d: char) -> u32 {
    match d {
        'U' if n > 3 => n - 3,
        'D' if n < 7 => n + 3,
        'L' if n % 3 != 1 => n - 1,
        'R' if n % 3 != 0 => n + 1,
        _ => n,
    }
}

// i feel dirty
fn part2_next(n: char, d: char) -> char {
    match d {
        'U' => match n {
            '3' => '1',
            '6' | '7' | '8' => char::from_digit(n.to_digit(10).unwrap() - 4, 10).unwrap(),
            'A' => '6',
            'B' => '7',
            'C' => '8',
            'D' => 'B',
            _ => n
         },
        'D' => match n {
            '1' => '3',
            '2' | '3' | '4' => char::from_digit(n.to_digit(10).unwrap() + 4, 10).unwrap(),
            '6' => 'A',
            '7' => 'B',
            '8' => 'C',
            'B' => 'D',
            _ => n,
        },
        'L' => match n {
            '3' | '4' | '6' | '7' | '8' | '9' => char::from_digit(n.to_digit(10).unwrap() - 1, 10).unwrap(),
            'B' => 'A',
            'C' => 'B',
            _ => n
        },
        'R' => match n {
            '2' | '3' | '5' | '6' | '7' | '8' => char::from_digit(n.to_digit(10).unwrap() + 1, 10).unwrap(),
            'A' => 'B',
            'B' => 'C',
            _ => n,
        },
        _ => panic!("invalid input"),
    }
}

fn main() {
    let mut part1 = String::new();
    let mut part2 = String::new();
    let mut p1 = 5;
    let mut p2 = '5';

    let stdin = io::stdin();
    let stdin = stdin.lock().lines();

    for line in stdin {
        let line = line.unwrap();
        for c in line.chars() {
            p1 = part1_next(p1, c);
            p2 = part2_next(p2, c);
        }
        part1.push(char::from_digit(p1, 10).unwrap());
        part2.push(p2);
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
