use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum State {
    Copy,
    Command,
    Collect(u32, u32),
}

fn main() {
    use State::*;

    let mut part1 = 0;

    let stdin = io::stdin();
    let stdin = stdin.lock().lines();
    for line in stdin {
        let line = line.unwrap();

        let mut output = String::new();
        let mut command = String::new();
        let mut repeat = String::new();
        let mut state = Copy;

        for chr in line.chars() {
            match state {
                Copy => {
                    if chr == '(' {
                        state = Command;
                    } else {
                        output.push(chr)
                    }
                }
                Command => {
                    if chr == ')' {
                        let tmp: Vec<_> =
                            command.split('x').map(|n| n.parse::<u32>().unwrap()).collect();
                        let (chars, repeat) = (tmp[0], tmp[1]);
                        state = Collect(chars, repeat);
                        command.clear();
                    } else {
                        command.push(chr);
                    }
                }
                Collect(n, r) => {
                    repeat.push(chr);
                    state = if n - 1 == 0 {
                        for _ in 0..r {
                            output.push_str(&repeat);
                        }
                        repeat.clear();
                        Copy
                    } else {
                        Collect(n - 1, r)
                    }

                }
            }
        }
        part1 += output.len();
    }
    println!("part 1: {}", part1);
}

