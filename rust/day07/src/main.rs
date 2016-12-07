use std::io;
use std::io::prelude::*;

fn main() {
    let abba = |text: &[u8]| {
        text.windows(4).any(|window|
           window[0] == window[3]
        && window[1] == window[2]
        && window[0] != window[1])
    };

    let aba_to_bab = |text: &[u8]| {
        text.windows(3)
            .filter(|w| w[0] == w[2] && w[0] != w[1])
            .map(|w| [w[1], w[0], w[1]])
            .collect::<Vec<_>>()
    };

    let tls = |supernet: &Vec<&str>, hypernet: &Vec<&str>| {
        supernet.iter().any(|text| abba(text.as_bytes())) &&
        hypernet.iter().all(|text| !abba(text.as_bytes()))
    };


    let mut part1 = 0;
    let mut part2 = 0;

    let stdin = io::stdin();
    let stdin = stdin.lock().lines();
    for line in stdin {
        let line = line.unwrap();
        let ip = line.split(|c| c == '[' || c == ']').enumerate();
        let mut hypernet: Vec<&str> = Vec::new();
        let mut supernet: Vec<&str> = Vec::new();
        for (i, txt) in ip {
            if i % 2 == 0 {
                supernet.push(txt);
            } else {
                hypernet.push(txt);
            }
        }

        if tls(&supernet, &hypernet) {
            part1 += 1;
        }

        let bab = supernet.iter()
            .flat_map(|part| aba_to_bab(part.as_bytes()))
            .map(|bytes: [u8; 3]| bytes.into_iter().map(|&b| b as char).collect::<String>())
            .collect::<Vec<String>>();

        if bab.iter().any(|triplet| hypernet.iter().any(|text| text.contains(&triplet.as_str()))) {
            part2 += 1;
        }
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
