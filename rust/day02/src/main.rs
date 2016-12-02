use std::io;
use std::io::prelude::*;

fn main() {
    let mut n = 5;
    let stdin = io::stdin();
    let stdin = stdin.lock().lines();
    for line in stdin {
        let line = line.unwrap();
        for c in line.chars() {
            n = match c {
                'U' if n > 3 => n - 3,
                'D' if n < 7 => n + 3,
                'L' if n % 3 != 1 => n - 1,
                'R' if n % 3 != 0 => n + 1,
                _ => n,
           }
        }
        print!("{}", n);
    }
    print!("\n");
}
