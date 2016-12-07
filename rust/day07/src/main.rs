use std::io;
use std::io::prelude::*;

fn main() {
    let abba = |text: &[u8]| text.windows(4).any(|window|
           window[0] == window[3]
        && window[1] == window[2]
        && window[0] != window[1]);

    let parse = |ip: Result<String, _>| {
        let ip = ip.unwrap();
        let mut even: Vec<String> = Vec::new();
        let mut odd: Vec<String> = Vec::new();
        for (i, txt) in ip.split(|c| c == '[' || c == ']').enumerate() {
            if i % 2 == 0 {
                even.push(txt.to_owned());
            } else {
                odd.push(txt.to_owned());
            }
        };
        (even, odd)
    };
    let tls = |&(ref even, ref odd) : &(Vec<String>, Vec<String>)|
               even.iter().any(|text| abba(text.as_bytes()))
            &&  odd.iter().all(|text| !abba(text.as_bytes()));

    let stdin = io::stdin();
    let stdin = stdin.lock().lines();
    let part1 = stdin.into_iter().map(parse).filter(tls).count();

    println!("part 1: {}", part1);
}
