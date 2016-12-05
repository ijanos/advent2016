extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::BTreeMap;

const INPUT: &'static str = "uqwqemis";

fn main() {
    let mut hasher = Md5::new();
    let hashes_with_five_zeroes = || {
        (0..)
            .map(move |i| {
                hasher.reset();
                hasher.input_str(&format!("{}{}", INPUT, i));
                hasher.result_str().to_owned()
            })
            .filter(|hash| hash.chars().take(5).all(|c| c == '0'))
    };
    let part1: String = hashes_with_five_zeroes()
        .take(8)
        .map(|hash| hash.chars().nth(5).unwrap())
        .collect();

    let mut part2 = BTreeMap::<char, char>::new();
    for hash in hashes_with_five_zeroes() {
        let index = hash.chars().nth(5).unwrap();
        if index.is_digit(10) && !part2.contains_key(&index) && index != '8' && index != '9' {
            part2.insert(index, hash.chars().nth(6).unwrap());
        }
        if part2.len() == 8 {
            break;
        }
    }
    println!("part 1: {}", part1);
    println!("part 2: {}", part2.values().cloned().collect::<String>());
}
