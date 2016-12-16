const INPUT: &'static str = "01111001100111011";
const PART1_LENGTH: usize = 272;
const PART2_LENGTH: usize = 35651584;

fn dragon_checksum(data: &str, disk_size: usize) -> String {
    let expand = |mut data: String| {
        let b = data.chars().rev().map(|c| if c == '1' { '0' } else { '1' }).collect::<String>();
        data.push('0');
        data.push_str(&b);
        data
    };

    let calc_checksum = |data: &String| {
        data.as_bytes()
            .chunks(2)
            .map(|pair| if pair[0] == pair[1] { '1' } else { '0' })
            .collect::<String>()
    };

    let mut data: String = data.into();
    while data.len() < disk_size {
        data = expand(data);
    }

    data.truncate(disk_size);
    let mut checksum = calc_checksum(&data);

    while checksum.len() % 2 == 0 {
        checksum = calc_checksum(&checksum);
    }

    checksum
}

fn main() {
    println!("part 1: {}", dragon_checksum(INPUT, PART1_LENGTH));
    println!("part 2: {}", dragon_checksum(INPUT, PART2_LENGTH));
}
