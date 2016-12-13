use std::collections::{HashMap, HashSet};

type Coord = (u16, u16);

fn main() {
    let mut wall_cache = HashMap::<Coord, bool>::new();

    let mut is_wall = |x, y| {
        if let Some(&wall) = wall_cache.get(&(x,y)) {
            return wall
        }
        let number = x*x + 3*x + 2*x*y + y + y*y + 1350;
        let ones = format!("{:b}", number).chars().filter(|&c| c == '1').count();
        let wall = ones % 2 != 0;
        wall_cache.insert((x,y),  wall);
        wall
    };

    let start = (1, 1);
    let goal = (31,39);
    let mut visited = HashSet::<Coord>::new();

    let mut steps = 0;
    let mut wave = vec![start];

    let part1;

    loop {
        steps += 1;

        if steps == 51 {
            println!("part 2: {}", visited.len());
        }

        let mut next_wave = Vec::new();
        for (x, y) in wave {
            if !is_wall(x+1, y) && !visited.contains(&(x+1, y)) {
                next_wave.push((x+1,y));
            }
            if !is_wall(x, y+1) && !visited.contains(&(x, y+1)) {
                next_wave.push((x,y+1));
            }
            if x > 0 && !is_wall(x-1, y) && !visited.contains(&(x-1, y)) {
                next_wave.push((x-1,y));
            }
            if y > 0 && !is_wall(x, y-1) && !visited.contains(&(x, y-1)) {
                next_wave.push((x,y-1));
            }
        }
        if next_wave.contains(&goal) {
            part1 = steps;
            break;
        }

        for &c in &next_wave {
            visited.insert(c);
        }

        wave = next_wave
    }
    println!("part 1: {}", part1);
}
