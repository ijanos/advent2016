fn main() {
    let cogs_p1 = |t| (t+1+11) % 13 + (t+2+0) % 5 + (t+3+11) % 17 + (t+4+0) % 3 + (t+5+2) % 7 + (t+6+17) % 19;
    let cogs_p2 = |t| (t+1+11) % 13 + (t+2+0) % 5 + (t+3+11) % 17 + (t+4+0) % 3 + (t+5+2) % 7 + (t+6+17) % 19 + (t+7+0) % 11;

    let part1 = (0..).filter(|&t| cogs_p1(t) == 0).next().unwrap();
    let part2 = (0..).filter(|&t| cogs_p2(t) == 0).next().unwrap();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}