use std::io;
use std::io::prelude::*;
use std::char;
use std::collections::HashMap;

fn part1_next(n: u32, d: char) -> u32 {
    match d {
        'U' if n > 3 => n - 3,
        'D' if n < 7 => n + 3,
        'L' if n % 3 != 1 => n - 1,
        'R' if n % 3 != 0 => n + 1,
        _ => n,
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const P2_KEYPAD: &'static [((i8, i8), char)] = &[
                            ((2,4), '1'),
              ((1,3), '2'), ((2,3), '3'), ((3,3), '4'),
((0,2), '5'), ((1,2), '6'), ((2,2), '7'), ((3,2), '8'), ((4,2), '9'),
              ((1,1), 'A'), ((2,1), 'B'), ((3,1), 'C'),
                            ((2,0), 'D')
];

#[cfg_attr(rustfmt, rustfmt_skip)]
fn translate_move(d: char) -> (i8, i8) {

    match d {
        'U' => ( 0,  1),
        'D' => ( 0, -1),
        'R' => ( 1,  0),
        'L' => (-1,  0),
        _ => panic!("invalid input"),
    }
}

fn main() {
    let mut part1 = String::new();
    let mut part2 = String::new();
    let mut p1 = 5;
    let mut p2_cur = (0, 2); // start at the coordinate of '5'

    let keypad: HashMap<(i8, i8), char> = P2_KEYPAD.iter().cloned().collect();
    let step = |(x1, y1), (x2, y2)| (x1 + x2, y1 + y2);

    let stdin = io::stdin();
    let stdin = stdin.lock().lines();
    for line in stdin {
        let line = line.unwrap();
        for c in line.chars() {
            p1 = part1_next(p1, c);
            let new_cur = step(p2_cur, translate_move(c));
            if keypad.contains_key(&new_cur) {
                p2_cur = new_cur;
            }
        }
        part1.push(char::from_digit(p1, 10).unwrap());
        part2.push(keypad[&p2_cur]);
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
