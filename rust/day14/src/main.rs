extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

const INPUT: &'static str = "jlmsuwbz";
use std::collections::VecDeque;

fn main() {
    let mut hasher = Md5::new();
    let mut hashes: VecDeque<(usize, String)> = VecDeque::with_capacity(1000);
    let mut i = 0;
    let mut key_i = 0;
    loop {
        if hashes.len() < 1000 {
            hasher.reset();
            hasher.input_str(&format!("{}{}", INPUT, i));
            hashes.push_back((i, hasher.result_str().to_owned()));
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
                        println!("part 1: {}", i);
                        break;
                    }
                }
            }
        }
    }
}
