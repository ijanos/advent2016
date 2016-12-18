extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

// Coordinates and current path
type State = (u8, u8, String);
const INPUT: &'static str = "hhhxzeay";

fn main() {
    let mut hasher = Md5::new();
    let mut md5 = |text: &str| {
        hasher.reset();
        hasher.input_str(&format!("{}{}", INPUT, text));
        hasher.result_str()
    };

    let open = |chr: char| chr.is_alphabetic() && chr != 'a';

    let mut open_doors = |path: &str| {
        let hash = md5(path);
        let doors = hash.chars().take(4).collect::<Vec<_>>();
        (open(doors[0]), open(doors[1]), open(doors[2]), open(doors[3]))
    };

    let start: State = (0, 0, "".into());

    let mut states = vec![start];
    let mut wins = Vec::<String>::new();

    loop {
        let mut new_states = Vec::new();
        for (x, y, path) in states {
            let (up, down, left, right) = open_doors(&path);
            if up && y != 0 {
                let mut p = path.clone();
                p.push('U');
                let s = (x, y - 1, p);
                new_states.push(s);
            }
            if down && y != 3 {
                let mut p = path.clone();
                p.push('D');
                let s = (x, y + 1, p);
                new_states.push(s);
            }
            if left && x != 0 {
                let mut p = path.clone();
                p.push('L');
                let s = (x - 1, y, p);
                new_states.push(s);
            }
            if right && x != 3 {
                let mut p = path.clone();
                p.push('R');
                let s = (x + 1, y, p);
                new_states.push(s);
            }
        }
        for &(x, y, ref p) in &new_states {
            if  x == 3 && y == 3 {
                wins.push(p.clone());
            }
        }
        states = new_states.into_iter().filter(|&(x, y, _)| !(x == 3 && y == 3)).collect();
        if states.is_empty() {
            break;
        }
    }

    println!("part 1: {}", wins[0]);
    println!("part 2: {}", wins.iter().max_by_key(|p| p.len()).unwrap().len());
}
