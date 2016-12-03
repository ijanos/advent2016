use std::io;
use std::io::prelude::*;


struct Triplets {
    column1: Vec<u32>,
    column2: Vec<u32>,
    column3: Vec<u32>,
    triangles: u32,
}

impl Triplets {
    fn new() -> Triplets {
        Triplets {
            column1: Vec::with_capacity(3),
            column2: Vec::with_capacity(3),
            column3: Vec::with_capacity(3),
            triangles: 0,
        }
    }

    fn push_triplet(&mut self, a: u32, b: u32, c: u32) {
        let valid_triangle =
            |ref v: &Vec<u32>| v[0] + v[1] > v[2] && v[0] + v[2] > v[1] && v[1] + v[2] > v[0];
        for &mut (ref mut column, n) in &mut [(&mut self.column1, a),
                                              (&mut self.column2, b),
                                              (&mut self.column3, c)] {
            column.push(n);
            if column.len() == 3 {
                if valid_triangle(&column) {
                    self.triangles += 1;
                }
                column.clear();
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock().lines();
    let valid_triangle = |a, b, c| a + b > c && a + c > b && b + c > a;
    let mut part2 = Triplets::new();
    let mut count = 0;
    for line in stdin {
        let line = line.unwrap();
        let mut sides = line.split_whitespace();
        let a = sides.next().unwrap().parse::<u32>().unwrap();
        let b = sides.next().unwrap().parse::<u32>().unwrap();
        let c = sides.next().unwrap().parse::<u32>().unwrap();
        if valid_triangle(a, b, c) {
            count += 1;
        }
        part2.push_triplet(a, b, c);
    }
    println!("part 1: {}", count);
    println!("part 2: {}", part2.triangles);
}
