extern crate itertools;

use std::io;
use std::str;
use std::io::prelude::*;

use itertools::Itertools;

#[derive(Debug)]
struct Node {
    x: isize,
    y: isize,
    size: u32,
    used: u32,
}

impl Node {
    fn from_line(line: String) -> Node {
        let line: Vec<&str> = line.split_whitespace().collect();
        let parse_coord =
            |coord: &str| str::from_utf8(&coord.as_bytes()[1..]).unwrap().parse::<isize>().unwrap();

        let coords: Vec<isize> = line[0]
            .split('/')
            .last()
            .unwrap()
            .split('-')
            .skip(1)
            .map(parse_coord)
            .collect();

        let parse_space = |item: &str| {
            str::from_utf8(&item.as_bytes()[..item.len() - 1]).unwrap().parse::<u32>().unwrap()
        };
        let size = parse_space(line[1]);
        let used = parse_space(line[2]);

        Node {
            x: coords[0],
            y: coords[1],
            size: size,
            used: used,
        }
    }

    fn avail(&self) -> u32 {
        self.size - self.used
    }
}

fn viable_pair(&(n1, n2): &(&Node, &Node)) -> bool {
    n1.used > 0 && n1.used < n2.avail() || n2.used > 0 && n2.used < n1.avail()
}


fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock().lines();
    let nodes: Vec<Node> = stdin.skip(2)
        .map(Result::unwrap)
        .map(Node::from_line)
        .collect();

    let part1 = nodes.iter().tuple_combinations().filter(viable_pair).count();
    println!("part 1: {}", part1);
}
