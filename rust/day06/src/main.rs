use std::io;
use std::io::prelude::*;
use std::collections::HashMap;


fn main() {
    let mut counts = Vec::<HashMap<char, u32>>::new();
    for _ in 0..8 {
        counts.push(HashMap::new());
    }

    let stdin = io::stdin();
    let stdin = stdin.lock().lines();
    for line in stdin {
        let line = line.unwrap();
        for (i, c) in line.chars().enumerate() {
            *counts[i].entry(c).or_insert(0) += 1;
        }
    }
    let part1: String = counts.iter()
        .map(|hm| {
            hm.into_iter().fold(('_', 0), |(c1, max), (&c2, &count)| if count > max {
                (c2, count)
            } else {
                (c1, max)
            })

        })
        .map(|(c, _)| c)
        .collect();

    let part2: String = counts.iter()
        .map(|hm| {
            hm.into_iter().fold(('_', std::u32::MAX),
                                |(c1, min), (&c2, &count)| if count < min {
                                    (c2, count)
                                } else {
                                    (c1, min)
                                })

        })
        .map(|(c, _)| c)
        .collect();
    println!("{:?}", part1);
    println!("{:?}", part2);
}
