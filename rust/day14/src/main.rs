extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::VecDeque;

const INPUT: &'static str = "jlmsuwbz";


fn main() {
    println!("part 1: {}", solve(false));
    println!("part 2: {}", solve(true));
}

fn solve(part2: bool) -> usize {
    let mut hasher = Md5::new();
    let mut hashes: VecDeque<(usize, String)> = VecDeque::with_capacity(1000);
    let mut i = 0;
    let mut key_i = 0;
    loop {
        if hashes.len() < 1000 {
            hasher.reset();
            hasher.input_str(&format!("{}{}", INPUT, i));
            let mut md5 = hasher.result_str().to_owned();
            if part2 {
                for _ in 0..2016 {
                    hasher.reset();
                    hasher.input_str(&md5);
                    md5 = hasher.result_str().to_owned();
                }
            }
            hashes.push_back((i, md5));
            i += 1;
        } else {
            let (i, h) = hashes.pop_front().unwrap();
            if let Some(triplet) = h.chars()
                .collect::<Vec<_>>()
                .windows(3)
                .find(|w| w[0] == w[1] && w[1] == w[2]) {
                if hashes.iter().any(|&(_, ref h)| {
                    h.chars()
                        .collect::<Vec<_>>()
                        .windows(5)
                        .any(|w| {
                            w[0] == triplet[0] && w[0] == w[1] && w[1] == w[2] && w[2] == w[3] &&
                            w[3] == w[4]
                        })
                }) {
                    key_i += 1;
                    if key_i == 64 {
                        return i;
                    }
                }
            }
        }
    }
}
