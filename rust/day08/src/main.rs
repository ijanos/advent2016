use std::io;
use std::fmt;
use std::io::prelude::*;
use std::str::FromStr;
use std::collections::VecDeque;


#[derive(Debug)]
struct Display {
    data: VecDeque<VecDeque<bool>>,
    height: usize,
    width: usize,
}

#[derive(Debug)]
enum Command {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

impl Display {
    fn new(w: usize, h: usize) -> Display {
        let mut rows = VecDeque::new();
        for _ in 0..h {
            let mut row = VecDeque::new();
            for _ in 0..w {
                row.push_back(false);
            }
            rows.push_back(row)
        }
        Display {
            data: rows,
            height: h,
            width: w,
        }
    }

    fn command(&mut self, cmd: &str) {
        let cmd: Command = cmd.parse().unwrap();
        use Command::*;
        match cmd {
            Rect(x, y) => {
                for y in 0..y {
                    for x in 0..x {
                        self.data[y][x] = true;
                    }
                }
            }
            RotateRow(y, offset) => {
                for _ in 0..offset {
                    let last = self.data[y].pop_back().unwrap();
                    self.data[y].push_front(last);
                }
            }
            RotateCol(x, offset) => {
                for _ in 0..offset {
                    let mut new_column: VecDeque<bool> =
                        (0..self.height).map(|y| self.data[y][x]).collect();
                    let last = new_column.pop_back().unwrap();
                    new_column.push_front(last);
                    for (y, &pixel) in new_column.iter().enumerate() {
                        self.data[y][x] = pixel;
                    }
                }
            }
        }
    }

    fn lit_pixels(&self) -> usize {
        self.data.iter().flat_map(|row| row.iter().filter(|&pixel| *pixel)).count()
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pixels = self.data
            .iter()
            .map(|row| row.iter().map(|&pixel| if pixel { '#' } else { '.' }).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", pixels)
    }
}


impl FromStr for Command {
    // Slice patterns would make this so much better
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Command, Self::Err> {
        let input = input.split_whitespace().collect::<Vec<_>>();
        match input[0] {
            "rect" => {
                let xy = input[1].split('x').collect::<Vec<_>>();
                let x = xy[0].parse().unwrap();
                let y = xy[1].parse().unwrap();
                Ok(Command::Rect(x, y))
            }
            "rotate" => {
                let coord = input[2].split('=').last().unwrap().parse().unwrap();
                let offset = input[4].parse().unwrap();
                match input[1] {
                    "row" => Ok(Command::RotateRow(coord, offset)),
                    "column" => Ok(Command::RotateCol(coord, offset)),
                    _ => Err("invalid command"),
                }
            }
            _ => Err("invalid command"),
        }

    }
}

fn main() {
    let mut display = Display::new(50, 6);

    let stdin = io::stdin();
    let stdin = stdin.lock().lines();

    for line in stdin {
        let line = line.unwrap();
        display.command(&line);
    }

    println!("part1: {}", display.lit_pixels());
    println!("part2:\n{}", display);
}
