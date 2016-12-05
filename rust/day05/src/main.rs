extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

const INPUT: &'static str = "uqwqemis";

fn main() {
    let mut hasher = Md5::new();
    let input = INPUT.as_bytes();
    for i in (0..std::u64::MAX) {
        hasher.input(input);
        hasher.input(i.to_string().as_bytes());
        let result = hasher.result_str();
        if result.chars().take(5).all(|c| c == '0') {
            println!("{}", result);
        }
        hasher.reset();
    }
    hasher.reset()
}
