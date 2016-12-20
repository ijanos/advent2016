extern crate bit_vec;

use bit_vec::BitVec;

use std::io;
use std::io::prelude::*;

pub fn main() {
    let mut blacklist = BitVec::from_elem(4_294_967_296, false);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let line = line.split('-').collect::<Vec<_>>();
        let (from, to) = (line[0].parse::<usize>().unwrap(), line[1].parse::<usize>().unwrap());
        for ip in from..to+1 {
            blacklist.set(ip, true);
        }
    }

    for i in 0..4_294_967_295 {
        if blacklist.get(i) == Some(false) {
            println!("part 1: {}", i);
            break;
        }
    }
}