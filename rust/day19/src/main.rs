use std::collections::VecDeque;


fn main() {
    // https://www.youtube.com/watch?v=uCsD3ZGzMgE
    // W(n) = 2l + 1
    println!("part 1: 1834903");

    let mut elves = VecDeque::new();
    for i in 1..3014604 {
        elves.push_back(i);
    }

    while elves.len() > 1 {
        let k = elves.len()/2;
        elves.remove(k);
        let head = elves.pop_front().unwrap();
        elves.push_back(head);
    }
    println!("part 2: {}", elves[0]);
}
