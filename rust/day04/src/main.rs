use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn decode_name(name: &str) -> String {
    let id: String = name.chars().filter(|&c| c.is_numeric()).collect();
    let id = id.parse::<u32>().unwrap();
    let rotate = |c: char, n| ((c as u32 - 97 + n) % 26 + 97) as u8 as char;
    name.chars()
        .filter(|&c| !c.is_numeric())
        .map(|c| if c == '-' { ' ' } else { rotate(c, id) })
        .collect()
}

fn main() {
    let mut letters = HashMap::<char, i32>::new();
    let mut sum = 0;
    let stdin = io::stdin();
    let stdin = stdin.lock().lines();
    for line in stdin {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(|c| c == '[' || c == ']').collect();
        let name = line[0];
        let checksum = line[1];
        for c in name.chars().filter(|&c| c.is_alphabetic()) {
            *letters.entry(c).or_insert(0) += 1;
        }
        let mut letter_freq: Vec<(i32, char)> = letters.iter().map(|(&c, &n)| (-n, c)).collect();
        letter_freq.sort_by(|l1, l2| l1.cmp(l2));
        let real_checksum = letter_freq.into_iter().take(5).map(|(_, c)| c).collect::<String>();
        if real_checksum == checksum {
            let id: String = name.chars().filter(|&c| c.is_numeric()).collect();
            let id = id.parse::<u32>().unwrap();
            sum += id;
            if decode_name(name) == "northpole object storage " {
                println!("part2: {}", id);
            }
        }
        letters.clear();
    }
    println!("part1: {}", sum);
}
