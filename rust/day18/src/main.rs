const INPUT: &'static str = ".^.^..^......^^^^^...^^^...^...^....^^.^...^.^^^^....^...^^.^^^...\
                             ^^^^.^^.^.^^..^.^^^..^^^^^^.^^^..^";

fn next_row(row: &[u8]) -> String {
    let mut next = String::with_capacity(row.len());
    let title = |left, center, right| match (left, center, right) {
        (b'^', b'^', b'.') => '^',
        (b'.', b'^', b'^') => '^',
        (b'^', b'.', b'.') => '^',
        (b'.', b'.', b'^') => '^',
        _ => '.',
    };
    // first title
    next.push(title(b'.', row[0], row[1]));

    for i in 1..row.len() - 1 {
        next.push(title(row[i - 1], row[i], row[i + 1]));
    }

    // last title
    next.push(title(row[row.len() - 2], row[row.len() - 2], b'.'));
    next
}

fn safe_titles(first_row: &str, lines: usize) -> usize {
    let mut safe = 0;
    let mut row = first_row.to_owned();
    for _ in 0..lines {
        safe += row.chars().filter(|&c| c == '.').count();
        row = next_row(row.as_bytes());
    }
    safe
}

fn main() {
    let part1 = safe_titles(INPUT, 40);
    let part2 = safe_titles(INPUT, 400_000);
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
