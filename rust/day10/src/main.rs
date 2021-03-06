use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
enum Node {
    Bot(u32),
    Output(u32),
}

#[derive(Debug)]
struct Bot {
    high: Node,
    low: Node,
    data: Vec<u32>,
}

fn parse_node(name: &str, index: &str) -> Node {
    match name {
        "bot" => Node::Bot(index.parse().unwrap()),
        "output" => Node::Output(index.parse().unwrap()),
        _ => panic!("invalid node"),
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().map(|line| line.unwrap()).collect::<Vec<String>>();

    let mut bots = HashMap::<u32, Bot>::new();
    let mut outputs = HashMap::<u32, u32>::new();

    for desc in input.iter().filter(|&line| line.starts_with("bot")) {
        let desc: Vec<_> = desc.split_whitespace().collect();
        let bot_id: u32 = desc[1].parse().unwrap();
        let low = parse_node(desc[5], desc[6]);
        let high = parse_node(desc[10], desc[11]);
        let data = Vec::new();
        let bot = Bot {
            high: high,
            low: low,
            data: data,
        };
        bots.insert(bot_id, bot);
    }

    for value in input.iter().filter(|&line| line.starts_with("value")) {
        let value: Vec<_> = value.split_whitespace().collect();
        let chip: u32 = value[1].parse().unwrap();
        let bot: u32 = value[5].parse().unwrap();
        bots.get_mut(&bot).unwrap().data.push(chip);
    }

    loop {
        let mut new_data: Vec<(u32, u32)> = Vec::new();
        if let (Some(a), Some(b), Some(c)) = (outputs.get(&0), outputs.get(&1), outputs.get(&2)) {
            println!("part 2: {}", a * b * c);
            break;
        }

        for (id, bot) in bots.iter_mut().filter(|&(_, ref bot)| bot.data.len() == 2) {
            bot.data.sort();
            if bot.data[0] == 17 && bot.data[1] == 61 {
                println!("part 1: {}", id);
            }
            let mut forward = |node, value| match node {
                &Node::Bot(id) => {
                    new_data.push((id, value));
                }
                &Node::Output(id) => {
                    outputs.insert(id, value);
                }
            };
            forward(&bot.low, bot.data[0]);
            forward(&bot.high, bot.data[1]);
            bot.data.clear();
        }
        for &(id, value) in &new_data {
            bots.get_mut(&id).unwrap().data.push(value);
        }
    }
}
