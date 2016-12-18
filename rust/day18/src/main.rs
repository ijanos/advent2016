const INPUT:  &'static str= ".^.^..^......^^^^^...^^^...^...^....^^.^...^.^^^^....^...^^.^^^...^^^^.^^.^.^^..^.^^^..^^^^^^.^^^..^";

fn next_row(row: &[u8]) -> String {
    let mut next = String::with_capacity(row.len());
    let title = |left, center, right| match (left, center,right) {
            (b'^',b'^',b'.') => '^',
            (b'.',b'^',b'^') => '^',
            (b'^',b'.',b'.') => '^',
            (b'.',b'.',b'^') => '^',
            _ => '.'
    };
    // first title
    next.push(title(b'.', row[0], row[1]));
    for i in 1..row.len() - 1  {
        next.push(title(row[i-1], row[i], row[i+1]));
    }
    // last title
    next.push(title(row[row.len()-2], row[row.len()-2], b'.'));
    next
}

fn main() {
    let mut row = INPUT.to_owned();
    let mut safe = 0;
    for _ in 0..40 {
        safe += row.chars().filter(|&c| c == '.').count();
        row = next_row(row.as_bytes());
    }
    println!("part 1: {}", safe);
}
